//! Rust module generation

use {
    crate::{
        boom::{
            self,
            passes::{
                self, cycle_finder::CycleFinder, fold_unconditionals::FoldUnconditionals,
                make_exception_panic::MakeExceptionPanic,
                monomorphize_vectors::MonomorphizeVectors, remove_const_branch::RemoveConstBranch,
                resolve_return_assigns::ResolveReturns,
            },
            Ast,
        },
        brig::{
            allowlist::apply_fn_allowlist,
            bits::codegen_bits,
            functions_interpreter::{codegen_block, codegen_parameters, get_block_fn_ident},
            state::codegen_state,
            workspace::{create_manifest, write_workspace},
        },
        rudder::{
            self, analysis::cfg::FunctionCallGraphAnalysis, Context, Function, PrimitiveTypeClass,
            Symbol, Type,
        },
    },
    cargo_util_schemas::manifest::{TomlManifest, TomlWorkspace},
    common::{create_file, intern::InternedString, HashMap, HashSet},
    log::{info, warn},
    once_cell::sync::Lazy,
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    rayon::iter::{IntoParallelIterator, ParallelIterator},
    regex::Regex,
    sailrs::{jib_ast, types::ListVec},
    std::{
        hash::{DefaultHasher, Hash, Hasher},
        io::Write,
        path::PathBuf,
        sync::Arc,
    },
    syn::Ident,
};

mod allowlist;
pub mod bits;
mod functions_dbt;
mod functions_interpreter;
mod state;
mod workspace;

const ENTRYPOINT: &str = "__DecodeA64";

/// Compiles a Sail model to a Brig module
pub fn sail_to_brig(
    jib_ast: ListVec<jib_ast::Definition>,
    path: PathBuf,
    dump_ir: Option<PathBuf>,
) {
    if let Some(_) = &dump_ir {
        sailrs::jib_ast::pretty_print::print_ast(jib_ast.iter());
    }

    info!("Converting JIB to BOOM");
    let ast = Ast::from_jib(apply_fn_allowlist(jib_ast.into_iter()));

    // // useful for debugging
    if let Some(path) = &dump_ir {
        boom::pretty_print::print_ast(
            &mut create_file(path.join("ast_raw.boom")).unwrap(),
            ast.clone(),
        );
    }

    info!("Running passes on BOOM");
    passes::run_fixed_point(
        ast.clone(),
        &mut [
            FoldUnconditionals::new_boxed(),
            RemoveConstBranch::new_boxed(),
            ResolveReturns::new_boxed(),
            MakeExceptionPanic::new_boxed(),
            MonomorphizeVectors::new_boxed(),
            CycleFinder::new_boxed(),
        ],
    );

    if let Some(path) = &dump_ir {
        boom::pretty_print::print_ast(
            &mut create_file(path.join("ast_processed.boom")).unwrap(),
            ast.clone(),
        );
    }

    info!("Building rudder");

    let mut rudder = rudder::build::from_boom(&ast.get());

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file(path.join("ast.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Validating rudder");
    let msgs = rudder.validate();
    for msg in msgs {
        warn!("{msg}");
    }

    info!("Optimising rudder");
    rudder.optimise(rudder::opt::OptLevel::Level3);

    if let Some(path) = &dump_ir {
        writeln!(
            &mut create_file(path.join("ast.opt.rudder")).unwrap(),
            "{rudder}"
        )
        .unwrap();
    }

    info!("Validating rudder again");
    let msgs = rudder.validate();
    for msg in msgs {
        warn!("{msg}");
    }

    info!("Generating Rust");
    let ws = codegen_workspace(&rudder);

    info!("Writing workspace to {:?}", &path);
    write_workspace(ws, path);
}

fn promote_width(width: usize) -> usize {
    match width {
        0 => 0,
        1..=8 => 8,
        9..=16 => 16,
        17..=32 => 32,
        33..=64 => 64,
        65..=128 => 128,
        width => {
            warn!("unsupported width: {width}");
            64
        }
    }
}

pub fn codegen_type(typ: Arc<Type>) -> TokenStream {
    match &*typ {
        Type::Primitive(typ) => {
            if typ.type_class() == PrimitiveTypeClass::UnsignedInteger && typ.width() == 1 {
                return quote!(bool);
            }

            let width = promote_width(typ.width());

            let rust_type = match typ.type_class() {
                PrimitiveTypeClass::Void => return quote!(()),
                PrimitiveTypeClass::Unit => return quote!(()),
                PrimitiveTypeClass::UnsignedInteger => {
                    format_ident!("u{}", width)
                }
                PrimitiveTypeClass::SignedInteger => {
                    format_ident!("i{}", width)
                }
                PrimitiveTypeClass::FloatingPoint => {
                    format_ident!("f{}", width)
                }
            };

            quote!(#rust_type)
        }
        Type::Composite(t) => {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            let hashed = format_ident!("CompositeType{:x}", hasher.finish());
            quote! { #hashed }
        }
        Type::Vector {
            element_count,
            element_type,
        } => {
            let element_type = codegen_type(element_type.clone());

            if *element_count == 0 {
                quote!(alloc::vec::Vec<#element_type>)
            } else {
                let count = quote!(#element_count);
                quote!([#element_type; #count])
            }
        }
        Type::Bits => {
            quote!(Bits)
        }
        Type::ArbitraryLengthInteger => quote!(i128),
    }
}

fn codegen_member(idx: usize) -> Ident {
    Ident::new(&format!("_{idx}"), Span::call_site())
}

pub fn codegen_ident(input: InternedString) -> Ident {
    static VALIDATOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap());

    let s = input.as_ref();

    if s == "main" {
        return Ident::new("model_main", Span::call_site());
    }

    let mut buf = String::with_capacity(s.len());

    for ch in s.chars() {
        match ch {
            '%' => buf.push_str("_pcnt_"),
            '&' => buf.push_str("_ref_"),
            '?' => buf.push_str("_unknown_"),
            '-' | '<' | '>' | '#' | ' ' | '(' | ')' | ',' | '\'' => buf.push('_'),
            _ => buf.push(ch),
        }
    }

    if buf.starts_with('_') {
        buf = "u".to_owned() + &buf;
    }

    if !VALIDATOR.is_match(&buf) {
        panic!("identifier {buf:?} not normalized even after normalizing");
    }

    Ident::new(&buf, Span::call_site())
}

fn codegen_types(rudder: &Context) -> TokenStream {
    let structs: TokenStream = rudder
        .get_structs()
        .into_iter()
        .map(|typ| {
            let ident = codegen_type(typ.clone());

            let Type::Composite(fields) = &*typ else {
                panic!("struct must be composite type");
            };

            let fields: TokenStream = fields
                .iter()
                .enumerate()
                .map(|(i, typ)| {
                    let name = codegen_member(i);
                    let typ = codegen_type(typ.clone());
                    quote!(pub #name: #typ,)
                })
                .collect();

            quote! {
                #[derive(Default, Debug, Clone, Copy)]
                #[repr(C)]
                pub struct #ident {
                    #fields
                }
            }
        })
        .collect();

    let unions: TokenStream = rudder
        .get_unions()
        .into_iter()
        .map(|typ| {
            let ident = codegen_type(typ.clone());

            let Type::Composite(fields) = &*typ else {
                panic!("union must be composite type");
            };

            let variants: TokenStream = fields
                .iter()
                .enumerate()
                .map(|(i, typ)| {
                    let name = codegen_member(i);
                    let typ = codegen_type(typ.clone());
                    quote!(#name(#typ),)
                })
                .collect();

            quote! {
                #[derive(Debug, Clone, Copy)]
                pub enum #ident {
                    #variants
                }

                impl Default for #ident {
                    fn default() -> Self {
                        Self::_0(Default::default())
                    }
                }
            }
        })
        .collect();

    quote! {
        #structs

        #unions
    }
}

fn codegen_workspace(rudder: &Context) -> (HashMap<PathBuf, String>, HashSet<PathBuf>) {
    // common crate depended on by all containing bundle, tracer, state, and
    // structs/enums/unions
    let common = {
        let header = codegen_header();
        let state = codegen_state(rudder);
        let bundle = codegen_bits();
        let types = codegen_types(rudder);

        (
            InternedString::from_static("common"),
            (
                HashSet::<InternedString>::default(),
                tokens_to_string(&quote! {
                    #header

                    #state

                    #bundle

                    #types

                    pub trait Tracer {
                        fn begin(&self, instruction: u32, pc: u64);
                        fn end(&self);
                        fn read_register<T: core::fmt::Debug>(&self, offset: isize, value: T);
                        fn write_register<T: core::fmt::Debug>(&self, offset: isize, value: T);
                    }

                    #[derive(Debug)]
                    pub enum ExecuteResult {
                        Ok,
                        EndOfBlock,
                        UndefinedInstruction
                    }
                }),
            ),
        )
    };

    // one top-level crate containing prelude
    let arch = {
        let header = codegen_header();
        let entrypoint_ident = codegen_ident(ENTRYPOINT.into());
        (
            InternedString::from_static("arch"),
            (
                [
                    InternedString::from_static("common"),
                    entrypoint_ident.to_string().into(),
                ]
                .into_iter()
                .collect::<HashSet<_>>(),
                tokens_to_string(&quote! {
                        #header

                        pub use common::*;

                        use #entrypoint_ident::#entrypoint_ident;

                        pub fn decode_execute<T: Tracer>(value: u32, state: &mut State, tracer: &T) -> ExecuteResult {
                            // reset SEE
                            state.write_register(REG_SEE, 0u64);

                            tracer.begin(value, state.read_register::<u64>(REG_U_PC));

                            #entrypoint_ident(state, tracer, i128::from(state.read_register::<u64>(REG_U_PC)), value);

                            // increment PC if no branch was taken
                            if !state.read_register::<bool>(REG_U__BRANCHTAKEN) {
                                let pc = state.read_register::<u64>(REG_U_PC);
                                state.write_register(REG_U_PC, pc + 4);
                            }

                            state.write_register(REG_U__BRANCHTAKEN, false);

                            tracer.end();

                            ExecuteResult::Ok
                        }

                }),
            ),
        )
    };

    rudder.update_names();
    let cfg = FunctionCallGraphAnalysis::new(rudder);
    let rudder_fns = rudder.get_functions();

    let crate_names = rudder_fns
        .keys()
        .copied()
        .chain(
            ["common", "arch"]
                .into_iter()
                .map(InternedString::from_static),
        )
        .map(|name| InternedString::from(codegen_ident(name).to_string()));

    let workspace_manifest = (
        PathBuf::from("Cargo.toml"),
        toml::to_string_pretty(&TomlManifest {
            cargo_features: None,
            package: None,
            project: None,
            profile: None,
            lib: None,
            bin: None,
            example: None,
            test: None,
            bench: None,
            dependencies: None,
            dev_dependencies: None,
            dev_dependencies2: None,
            build_dependencies: None,
            build_dependencies2: None,
            features: None,
            target: None,
            replace: None,
            patch: None,
            workspace: Some(TomlWorkspace {
                members: Some(crate_names.clone().map(|s| s.to_string()).collect()),
                resolver: Some("2".to_owned()),
                exclude: None,
                default_members: None,
                metadata: None,
                package: None,
                dependencies: None,
                lints: None,
            }),
            badges: None,
            lints: None,
        })
        .unwrap(),
    );

    let dirs = crate_names
        .map(|name| [PathBuf::from(name.as_ref()).join("src")].into_iter())
        .flatten()
        .collect();

    let files = rudder_fns.into_par_iter()
        .map(|(name, function)| {
            let name_ident = codegen_ident(name);
            let (return_type, parameters) = function.signature();

            let function_parameters = codegen_parameters(&parameters);
            let return_type = codegen_type(return_type);

            let fn_state = codegen_fn_state(&function, parameters);

            let entry_block = get_block_fn_ident(&function.entry_block());

            let block_fns = function
                .entry_block()
                .iter()
                .map(|block| {
                    let block_name = get_block_fn_ident(&block);
                    let block_impl = codegen_block(block);

                    quote! {
                        fn #block_name<T: Tracer>(state: &mut State, tracer: &T, mut fn_state: FunctionState) -> #return_type {
                            #block_impl
                        }
                    }
                })
                .collect::<TokenStream>();

            let contents =
                quote! {
                    pub fn #name_ident<T: Tracer>(#function_parameters) -> #return_type {
                        #fn_state

                        return #entry_block(state, tracer, fn_state);

                        #block_fns
                    }
                };

            let mut dependencies = cfg.get_callees_for(&name);
            dependencies.push("common".into());

            let imports: TokenStream = dependencies
                .iter()
                .map(|krate| {
                    let krate = codegen_ident(*krate);
                    quote!(use #krate::*;)
                })
                .collect();

            let dependencies = dependencies
                .into_iter()
                .map(|name| InternedString::from(codegen_ident(name).to_string()))
                .collect::<HashSet<_>>();

            let header = codegen_header();

            (
                InternedString::from(codegen_ident(name).to_string()),
               (
                    dependencies,
                    tokens_to_string(&quote! {
                        #header

                        extern crate alloc;
                        #imports

                        #contents
                    }),
               )
            )
        })
        .chain([arch, common])

        .map(|(name, (dependencies, contents))| {
            let manifest = (
                PathBuf::from(name.as_ref()).join("Cargo.toml"),
                toml::to_string(&create_manifest(name, &dependencies)).unwrap(),
            );

            let source = (
                PathBuf::from(name.as_ref()).join("src").join("lib.rs"),
                contents,
            );

            [manifest, source]
        })
        .flatten()
        .chain([workspace_manifest])
        .collect();

    (files, dirs)
}

fn codegen_fn_state(function: &Function, parameters: Vec<Symbol>) -> TokenStream {
    let fn_state = {
        let fields = function
            .local_variables()
            .iter()
            .chain(&parameters)
            .map(|symbol| {
                let name = codegen_ident(symbol.name());
                let typ = codegen_type(symbol.typ());

                quote! {
                    #name: #typ,
                }
            })
            .collect::<TokenStream>();

        // copy from parameters into fn state
        let parameter_copies = parameters
            .iter()
            .map(|symbol| {
                let name = codegen_ident(symbol.name());

                quote! {
                    #name,
                }
            })
            .collect::<TokenStream>();

        quote! {
            #[derive(Default)]
            struct FunctionState {
                #fields
            }

            let fn_state = FunctionState {
                #parameter_copies
                ..Default::default()
            };
        }
    };
    fn_state
}

pub fn tokens_to_string(tokens: &TokenStream) -> String {
    let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    // fix comments
    formatted.replace("///", "//")
}

fn codegen_header() -> TokenStream {
    quote! {
        #![no_std]
        #![allow(non_snake_case)]
        #![allow(unused_assignments)]
        #![allow(unused_mut)]
        #![allow(unused_parens)]
        #![allow(unused_variables)]
        #![allow(dead_code)]
        #![allow(unreachable_code)]
        #![allow(unused_doc_comments)]
        #![allow(non_upper_case_globals)]


        //! BOREALIS GENERATED FILE
    }
}
