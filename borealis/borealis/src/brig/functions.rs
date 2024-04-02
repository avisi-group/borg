//! Rust code generation
//!
//! Watch out for `return quote!(...)` in these functions when they build up
//! quotes

use {
    crate::rudder::{
        BinaryOperationKind, Block, CastOperationKind, ConstantValue, Context, Function,
        PrimitiveTypeClass, ShiftOperationKind, Statement, StatementKind, Symbol, Type,
        UnaryOperationKind,
    },
    common::{intern::InternedString, HashSet},
    log::warn,
    once_cell::sync::Lazy,
    proc_macro2::{Ident, Literal, Span, TokenStream},
    quote::{format_ident, quote, ToTokens},
    regex::Regex,
    std::{
        borrow::Borrow,
        hash::{DefaultHasher, Hash, Hasher},
        rc::Rc,
    },
};

pub fn codegen_functions(rudder: Context, entrypoint: InternedString) -> TokenStream {
    rudder.update_names();

    let fn_names = get_functions_to_codegen(&rudder, entrypoint);

    let rudder_fns = rudder.get_functions();

    let fns: TokenStream = fn_names
        .into_iter()
        .map(|k| (k, rudder_fns.get(&k).unwrap()))
        .map(|(name, function)| {
            let name = codegen_ident(name);
            let (return_type, parameters) = function.signature();
            let return_type = codegen_type(return_type);
            let parameters = codegen_parameters(parameters);
            let body = codegen_body(function.clone());

            quote! {
                fn #name<T: Tracer>(#parameters) -> #return_type {
                    #body
                }
            }
        })
        .collect();

    let structs: TokenStream = rudder
        .get_structs()
        .into_iter()
        .map(|typ| {
            let ident = codegen_type(typ.clone());

            let Type::Composite(fields) = typ.borrow() else {
                panic!("struct must be composite type");
            };

            let fields: TokenStream = fields
                .iter()
                .enumerate()
                .map(|(i, typ)| {
                    let name = codegen_member(i);
                    let typ = codegen_type(typ.clone());
                    quote!(#name: #typ,)
                })
                .collect();

            quote! {
                #[derive(Default, Debug, Clone, Copy)]
                struct #ident {
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

            let Type::Composite(fields) = typ.borrow() else {
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
                enum #ident {
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

    let entrypoint = codegen_ident(entrypoint);
    let entrypoint = quote!(#entrypoint(state, tracer, Bundle { value: state.read_register(REG_U_PC), length: 64 }, value););

    quote! {
        #structs

        #unions

        fn decode_execute<T: Tracer>(value: u32, state: &mut State, tracer: &mut T) -> ExecuteResult {
            // reset SEE
            // todo: for the love of god make this not break if registers are regenerated
            state.write_register(REG_SEE, 0u64);

            #entrypoint

            // increment PC if no branch was taken
            let branch_taken = state.read_register::<bool>(REG_U__BRANCHTAKEN);

            if !branch_taken {
                let pc = state.read_register::<u64>(REG_U_PC);
                state.write_register(REG_U_PC, pc + 4);
            }

            state.write_register(REG_U__BRANCHTAKEN, false);

            ExecuteResult::Ok
        }

        #fns
    }
}

fn codegen_parameters(parameters: Vec<Symbol>) -> TokenStream {
    let parameters = [quote!(state: &mut State, tracer: &mut T)]
        .into_iter()
        .chain(parameters.iter().map(|sym| {
            let name = codegen_ident(sym.name());
            let typ = codegen_type(sym.typ());
            quote!(#name: #typ)
        }));

    quote! {
        #(#parameters),*
    }
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

pub fn codegen_type(typ: Rc<Type>) -> TokenStream {
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
            quote! {#hashed}
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
        Type::Bundled { value, len } => {
            let value_type = codegen_type(value.clone());
            let len_type = codegen_type(len.clone());
            quote!(Bundle<#value_type, #len_type>)
        }
    }
}

fn codegen_body(function: Function) -> TokenStream {
    // local variables
    // block indicies
    // current block

    let block_defs = function
        .entry_block()
        .iter()
        .enumerate()
        .map(|(idx, block)| {
            let block_name = format_ident!("BLOCK_{}", block.name().to_string());
            let idx = u32::try_from(idx).unwrap();

            quote! {
                const #block_name: u32 = #idx;
            }
        })
        .collect::<TokenStream>();

    let local_vars = function
        .local_variables()
        .iter()
        .map(|symbol| {
            let name = codegen_ident(symbol.name());
            let typ = codegen_type(symbol.typ());

            quote! {
                let mut #name: #typ = Default::default();
            }
        })
        .collect::<TokenStream>();

    let entry_block_name = format_ident!("BLOCK_{}", function.entry_block().name().to_string());

    let block_arms = function
        .entry_block()
        .iter()
        .map(|block| {
            let block_name = format_ident!("BLOCK_{}", block.name().to_string());
            let block_impl = codegen_block(block);

            quote! {
                #block_name => {
                    #block_impl
                }
            }
        })
        .collect::<TokenStream>();

    quote! {
        #block_defs

        #local_vars

        let mut current_block = #entry_block_name;

        loop {
            match current_block {
                #block_arms
                u => panic!("undefined block {u}")
            }
        }
    }
}

fn codegen_block(block: Block) -> TokenStream {
    block
        .statements()
        .iter()
        .cloned()
        .map(codegen_stmt)
        .collect()
}

fn get_ident(stmt: &Statement) -> TokenStream {
    format_ident!("{}", stmt.name().to_string()).to_token_stream()
}

fn codegen_stmt(stmt: Statement) -> TokenStream {
    let stmt_name = format_ident!("{}", stmt.name().to_string());

    let value = match stmt.kind() {
        StatementKind::Constant { value, typ } => {
            let v = match value {
                // todo: do not emit type info here
                ConstantValue::UnsignedInteger(v) => {
                    if *typ == Type::u1() {
                        let b = v != 0;
                        quote!(#b)
                    } else {
                        let v = Literal::usize_unsuffixed(v);
                        quote!(#v)
                    }
                }
                ConstantValue::SignedInteger(v) => {
                    let v = Literal::isize_unsuffixed(v);
                    quote!(#v)
                }
                ConstantValue::FloatingPoint(v) => {
                    let v = Literal::f64_unsuffixed(v);
                    quote!(#v)
                }

                ConstantValue::Unit => quote!(()),
            };

            if let Type::Bundled { .. } = &*stmt.typ() {
                let length = Literal::usize_unsuffixed(typ.width_bits());
                quote!(Bundle {value: #v, length: #length})
            } else {
                v
            }
        }
        StatementKind::ReadVariable { symbol } => {
            let var = codegen_ident(symbol.name());
            quote! {#var}
        }
        StatementKind::WriteVariable { symbol, value } => {
            let var = codegen_ident(symbol.name());
            let value = get_ident(&value);
            quote! {#var = #value}
        }
        StatementKind::ReadRegister { typ, offset } => {
            let offset = get_ident(&offset);
            let typ = codegen_type(typ);
            quote! {
                {
                    let value = state.read_register::<#typ>(#offset as isize);
                    tracer.read_register(#offset as usize, value);
                    value
                }
            }
        }
        StatementKind::WriteRegister { offset, value } => {
            let offset = get_ident(&offset);
            let typ = codegen_type(value.typ());
            let value = get_ident(&value);
            quote! {
                {
                    state.write_register::<#typ>(#offset as isize, #value);
                    tracer.write_register(#offset as usize, #value);
                }
            }
        }
        StatementKind::ReadMemory { .. } => quote!(todo!("read-mem")),
        StatementKind::WriteMemory { .. } => quote!(todo!("write-mem")),
        StatementKind::ReadPc => quote!(todo!("read-pc")),
        StatementKind::WritePc { .. } => quote!(todo!("write-pc")),
        StatementKind::BinaryOperation { kind, lhs, rhs } => {
            let mut left = get_ident(&lhs);
            let mut right = get_ident(&rhs);

            // hard to decide whether this belongs, but since it's a Rust issue that u1 is
            // not like other types, casting is a codegen thing
            match (lhs.typ().width_bits(), rhs.typ().width_bits()) {
                // both bools, do nothing
                (1, 1) => (),
                (1, _) => {
                    let typ = codegen_type(rhs.typ());
                    left = quote!(((#left) as #typ));
                }
                (_, 1) => {
                    let typ = codegen_type(lhs.typ());
                    right = quote!(((#right) as #typ));
                }
                // both not bools, do nothing
                (_, _) => (),
            }

            let op = match kind {
                BinaryOperationKind::CompareEqual => quote! { (#left) == (#right) },
                BinaryOperationKind::Add => quote! { (#left) + (#right) },
                BinaryOperationKind::Sub => quote! { (#left) - (#right) },
                BinaryOperationKind::Multiply => quote! { (#left) * (#right) },
                BinaryOperationKind::Divide => quote! { (#left) / (#right) },
                BinaryOperationKind::Modulo => quote! { (#left) % (#right) },
                BinaryOperationKind::And => quote! { (#left) & (#right) },
                BinaryOperationKind::Or => quote! { (#left) | (#right) },
                BinaryOperationKind::Xor => quote! { (#left) ^ (#right) },
                BinaryOperationKind::CompareNotEqual => quote! { (#left) != (#right) },
                BinaryOperationKind::CompareLessThan => quote! { (#left) < (#right) },
                BinaryOperationKind::CompareLessThanOrEqual => quote! { (#left) <= (#right) },
                BinaryOperationKind::CompareGreaterThan => quote! { (#left) > (#right) },
                BinaryOperationKind::CompareGreaterThanOrEqual => quote! { (#left) >= (#right) },
            };

            quote! { (#op) }
        }
        StatementKind::UnaryOperation { kind, value } => {
            let value = get_ident(&value);

            match kind {
                UnaryOperationKind::Not => quote! {!#value},
                UnaryOperationKind::Negate => quote! {-#value},
                UnaryOperationKind::Complement => quote! {!#value},
            }
        }
        StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => {
            let value = get_ident(&value);
            let amount = get_ident(&amount);

            match kind {
                ShiftOperationKind::LogicalShiftLeft => quote! {#value << #amount},
                ShiftOperationKind::LogicalShiftRight => quote! {#value >> #amount},
                ShiftOperationKind::ArithmeticShiftRight => {
                    quote! {#value >> #amount}
                }
                ShiftOperationKind::RotateRight => todo!(),
                ShiftOperationKind::RotateLeft => todo!(),
            }
        }
        StatementKind::Call { target, args } => {
            let ident = codegen_ident(target.name());
            let args = args.iter().map(get_ident);

            // todo: remove this
            if target.name().as_ref().starts_with("unimplemented_") {
                let msg = target.name().to_string();
                quote! { unimplemented!(#msg) }
            } else {
                quote! {
                    #ident(state, tracer, #(#args),*)
                }
            }
        }
        StatementKind::Cast { typ, value, kind } => {
            let source = value.typ();
            let target = typ;
            let ident = get_ident(&value);

            if source == target {
                // todo: check this never happens
                quote! {
                    ((#ident))
                }
            } else if target == Rc::new(Type::u1()) {
                // todo: is special casing booleans like this necessary?
                // todo: this is duplicated in decode
                quote! {
                    ((#ident) != 0)
                }
            } else if target.is_unknown_length_vector() {
                quote!(alloc::vec::Vec::from(#ident))
            } else {
                match kind {
                    CastOperationKind::ZeroExtend => {
                        // safe even if underlying rudder types are smaller than codegen'd rust types (u7 -> u13 == u8 -> u16)
                        let target = codegen_type(target);
                        quote! {
                            (#ident as #target)
                        }
                    }

                    CastOperationKind::Truncate => {
                        match (&*source, &*target) {
                            (Type::Primitive(_), Type::Primitive(_)) => {
                                assert!(target.width_bits() < source.width_bits());

                                // create mask of length target
                                let mask =
                                    Literal::u64_unsuffixed((1u64 << target.width_bits()) - 1);

                                // cast to target type and apply mask to source
                                let target = codegen_type(target);

                                quote!(((#ident as #target) & #mask))
                            }
                            (Type::Bundled { .. }, Type::Bundled { .. }) => {
                                panic!("cannot truncate bundles, target length not known by type system")
                            }
                            _ => todo!(),
                        }
                    }

                    CastOperationKind::SignExtend => todo!(),
                    CastOperationKind::Reinterpret => {
                        let target = codegen_type(target);
                        quote! {
                            (#ident as #target)
                        }
                    }
                    CastOperationKind::Convert => todo!(),
                    CastOperationKind::Broadcast => todo!(),
                }
            }
        }
        StatementKind::Jump { target } => {
            let target = format_ident!("BLOCK_{}", target.name().to_string());
            quote! {
                current_block = #target;
            }
        }
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => {
            let condition = get_ident(&condition);
            let true_target = format_ident!("BLOCK_{}", true_target.name().to_string());
            let false_target = format_ident!("BLOCK_{}", false_target.name().to_string());

            quote! {
                current_block = if #condition { #true_target } else { #false_target };
            }
        }
        StatementKind::PhiNode { .. } => quote!(todo!("phi")),
        StatementKind::Return { value } => match value {
            Some(value) => {
                let name = codegen_ident(value.name());
                quote! { return #name; }
            }
            None => {
                quote! { return; }
            }
        },
        StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => {
            let condition = get_ident(&condition);
            let true_value = get_ident(&true_value);
            let false_value = get_ident(&false_value);
            quote! { if #condition { #true_value } else { #false_value } }
        }
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => {
            if let Type::Bundled { .. } = &*value.typ() {
                let length = if let Type::Bundled { .. } = &*length.typ() {
                    let length = get_ident(&length);
                    quote!(#length.value as u32)
                } else {
                    let length = get_ident(&length);
                    quote!(#length as u32)
                };

                let value = get_ident(&value);
                let start = get_ident(&start);

                quote! (
                    (
                        (#value >> #start) &
                        Bundle { value: ((1u64).checked_shl(#length).map(|x| x - 1).unwrap_or(!0)), length: #value.length }
                    )
                )
            } else {
                let typ = codegen_type(value.typ());

                let value = get_ident(&value);
                let start = get_ident(&start);
                let length = get_ident(&length);

                // todo: pre-cast length to u32

                quote! (
                    (
                        (#value >> #start) &
                        ((1 as #typ).checked_shl(#length as u32).map(|x| x - 1).unwrap_or(!0))
                    )
                )
            }
        }
        StatementKind::BitInsert { .. } => quote!(todo!("bitins")),
        StatementKind::Panic(statements) => {
            let args = statements.iter().map(get_ident);

            quote!(panic!("{:?}", (#(#args),*)))
        }
        StatementKind::ReadField { composite, field } => {
            let composite = get_ident(&composite);
            let field = codegen_member(field);
            quote!(#composite.#field)
        }
        StatementKind::MutateField {
            composite,
            field,
            value,
        } => {
            let composite = get_ident(&composite);
            let field = format_ident!("_{field}");
            let value = get_ident(&value);
            quote! {
                {
                    let mut local = #composite.clone();
                    local.#field = #value;
                    local
                }
            }
        }
        StatementKind::ReadElement { vector, index } => {
            let index_typ = index.typ();

            let vector = get_ident(&vector);
            let index = get_ident(&index);

            if let Type::Bundled { .. } = &*index_typ {
                quote!(#vector[(#index.value) as usize])
            } else {
                quote!(#vector[(#index) as usize])
            }
        }
        StatementKind::MutateElement {
            vector,
            value,
            index,
        } => {
            let vector = get_ident(&vector);
            let index = get_ident(&index);
            let value = get_ident(&value);
            // todo: support bundle indexes
            quote! {
                {
                    let mut local = #vector.clone();
                    local[(#index) as usize] = #value;
                    local
                }
            }
        }
        StatementKind::CreateComposite { typ, fields } => {
            let typ = codegen_type(typ);
            let fields = fields
                .iter()
                .enumerate()
                .map(|(index, statement)| {
                    let field_name = codegen_member(index);
                    let value = get_ident(statement);
                    quote!(#field_name: #value,)
                })
                .collect::<TokenStream>();
            quote!(#typ { #fields })

            // struct { _0: foo, _1: bar }
        }
        StatementKind::Bundle { value, length } => {
            let value = get_ident(&value);
            let length = get_ident(&length);
            quote!(Bundle { value: #value, length: #length })
        }
        StatementKind::UnbundleValue { bundle } => {
            let bundle = get_ident(&bundle);
            quote!((#bundle.value))
        }
        StatementKind::UnbundleLength { bundle } => {
            let bundle = get_ident(&bundle);
            quote!((#bundle.length))
        }
        StatementKind::Assert { condition } => {
            let condition = get_ident(&condition);
            quote!(assert!(#condition))
        }
    };

    let msg = format!(" {stmt}");
    if stmt.has_value() {
        let typ = codegen_type(stmt.typ());
        quote! {
            #[doc = #msg]
            let #stmt_name: #typ = #value;
        }
    } else {
        quote! {
            #[doc = #msg]
            #value;
        }
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

fn get_functions_to_codegen(rudder: &Context, entrypoint: InternedString) -> Vec<InternedString> {
    let rudder_fns = rudder.get_functions();

    let mut fns = HashSet::default();
    let mut todo = vec![entrypoint];

    fn get_calls(f: &Function) -> Vec<InternedString> {
        f.entry_block()
            .iter()
            .flat_map(|b| b.statements().into_iter())
            .filter_map(|s| {
                if let StatementKind::Call { target, .. } = s.kind() {
                    Some(target.name())
                } else {
                    None
                }
            })
            .collect()
    }

    while let Some(next) = todo.pop() {
        if fns.contains(&next) {
            continue;
        }

        let next_children = get_calls(rudder_fns.get(&next).unwrap());
        todo.extend(next_children);
        fns.insert(next);
    }

    fns.into_iter().collect()
}
