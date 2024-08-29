//! Rust module generation

use {
    crate::{
        codegen::{
            dynamic::{codegen_block, codegen_parameters, get_block_fn_ident},
            state::codegen_state,
            workspace::create_manifest,
        },
        rudder::{
            analysis::cfg::FunctionCallGraphAnalysis, Context, Function, PrimitiveTypeClass,
            Symbol, Type,
        },
        GenerationMode,
    },
    cargo_util_schemas::manifest::{TomlManifest, TomlWorkspace},
    common::{intern::InternedString, HashMap, HashSet},
    log::warn,
    once_cell::sync::Lazy,
    proc_macro2::{Span, TokenStream},
    quote::{format_ident, quote},
    rayon::iter::{IntoParallelIterator, ParallelIterator},
    regex::Regex,
    std::{
        collections::BTreeSet,
        hash::{DefaultHasher, Hash, Hasher},
        path::PathBuf,
        sync::Arc,
    },
    syn::Ident,
};

pub mod dynamic;
pub mod interpreter;
pub mod state;
pub mod workspace;

// Rust source that will be emitted, but included here for compile checking +
// testing
mod include;

fn promote_width(width: usize) -> usize {
    match width {
        0..=8 => 8,
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
        Type::Struct(t) => {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            let hashed = format_ident!("Struct{:x}", hasher.finish());
            quote! { #hashed }
        }
        Type::Enum(t) => {
            let mut hasher = DefaultHasher::new();
            t.hash(&mut hasher);
            let hashed = format_ident!("Enum{:x}", hasher.finish());
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
        Type::Rational => {
            quote!(num_rational::Ratio<i128>)
        }
        Type::ArbitraryLengthInteger => quote!(i128),
        Type::String => quote!(&'static str),
        // maybe this should be `core::Any`?
        Type::Any => quote!(_),
    }
}

pub fn codegen_ident(input: InternedString) -> Ident {
    static VALIDATOR: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap());

    let s = input.as_ref();

    if s == "main" {
        return Ident::new("model_main", Span::call_site());
    } else if s == "break" {
        return Ident::new("_break", Span::call_site());
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

            let Type::Struct(fields) = &*typ else {
                panic!("struct must be product type");
            };

            let fields: TokenStream = fields
                .iter()
                .map(|(name, typ)| {
                    let name = codegen_ident(*name);
                    let typ = codegen_type(typ.clone());
                    quote!(pub #name: #typ,)
                })
                .collect();

            quote! {
                #[derive(Default, Debug, Clone, Copy, PartialEq)]
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

            let Type::Enum(fields) = &*typ else {
                panic!("union must be sum type");
            };

            let variants: TokenStream = fields
                .iter()
                .map(|(name, typ)| {
                    let name = codegen_ident(*name);
                    let typ = codegen_type(typ.clone());
                    quote!(#name(#typ),)
                })
                .collect();

            let first_field = codegen_ident(fields.iter().next().unwrap().0);

            quote! {
                #[derive(Debug, Clone, Copy, PartialEq)]
                pub enum #ident {
                    #variants
                }

                impl Default for #ident {
                    fn default() -> Self {
                        Self::#first_field(Default::default())
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

pub fn codegen_workspace(rudder: &Context) -> (HashMap<PathBuf, String>, HashSet<PathBuf>) {
    {
        let mut files = HashMap::default();

        rudder.update_names();
        let rudder_fns = rudder.get_functions();

        let funcs = rudder_fns
            .values()
            .map(dynamic::codegen_function)
            .collect::<TokenStream>();

        let state = codegen_state(rudder);
        let types = codegen_types(rudder);
        let bits = include::get("bits.rs");
        let util = include::get("util.rs");

        let contents = quote! {
            //! aarch64

            #![allow(non_snake_case)]
            #![allow(unused_assignments)]
            #![allow(unused_mut)]
            #![allow(unused_parens)]
            #![allow(unused_variables)]
            #![allow(unused_imports)]
            #![allow(dead_code)]
            #![allow(unreachable_code)]
            #![allow(unused_doc_comments)]
            #![allow(non_upper_case_globals)]
            #![allow(non_camel_case_types)]

            use crate::dbt::{
                emitter::{Emitter, Type, TypeKind, BlockResult},
                x86::{
                    emitter::{UnaryOperationKind, BinaryOperationKind,CastOperationKind, ShiftOperationKind, X86BlockRef, X86Emitter, X86NodeRef, X86SymbolRef},
                    X86TranslationContext,
                },
                TranslationContext,
            };

            #funcs

            #state

            #types

            #bits

            #util
        };

        files.insert("mod.rs".into(), render(&contents));
        return (files, HashSet::default());
    }

    // common crate depended on by all containing bundle, tracer, state, and
    // structs/enums/unions
    let common = {
        let header = codegen_header();
        let state = codegen_state(rudder);
        let types = codegen_types(rudder);
        let bits = include::get("bits.rs");
        let util = include::get("util.rs");

        (
            InternedString::from_static("common"),
            (
                HashSet::<InternedString>::default(),
                render(&quote! {
                    #header

                    #state

                    #types

                    #bits

                    #util
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
        .chain(["common"].into_iter().map(InternedString::from_static))
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
            _unused_keys: BTreeSet::new(),
        })
        .unwrap(),
    );

    let dirs = crate_names
        .flat_map(|name| [PathBuf::from(name.as_ref()).join("src")].into_iter())
        .collect();

    let files = rudder_fns
        .into_par_iter()
        .map(|(name, _function)| {
            let contents = quote!();

            let mut dependencies = cfg.get_callees_for(&name);
            dependencies.push("common".into());
            let dependencies = dependencies
                .into_iter()
                .filter(|dep| *dep != name)
                .collect::<Vec<_>>();

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
                    render(&quote! {
                        #header

                        #imports

                        #contents
                    }),
                ),
            )
        })
        .chain([common])
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

pub fn render(tokens: &TokenStream) -> String {
    let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    // fix comments
    formatted.replace("///", "//")
}

/// Header for all generated Rust files
fn codegen_header() -> TokenStream {
    quote! {
        #![no_std]

        #![allow(non_snake_case)]
        #![allow(unused_assignments)]
        #![allow(unused_mut)]
        #![allow(unused_parens)]
        #![allow(unused_variables)]
        #![allow(unused_imports)]
        #![allow(dead_code)]
        #![allow(unreachable_code)]
        #![allow(unused_doc_comments)]
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]

        //! BOREALIS GENERATED FILE

        extern crate alloc;
    }
}
