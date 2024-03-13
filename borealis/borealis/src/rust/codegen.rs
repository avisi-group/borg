//! Rust code generation
//!
//! Watch out for `return quote!(...)` in these functions when they build up
//! quotes

use {
    crate::rudder::{
        BinaryOperationKind, Block, ConstantValue, Context, Function, PrimitiveTypeClass,
        ShiftOperationKind, Statement, StatementKind, Symbol, Type, UnaryOperationKind,
    },
    proc_macro2::{Ident, Literal, Span, TokenStream},
    quote::{format_ident, quote, ToTokens},
    std::{
        borrow::Borrow,
        hash::{DefaultHasher, Hash, Hasher},
        rc::Rc,
    },
};

pub fn codegen(rudder: Context) -> TokenStream {
    rudder.update_names();

    let fns: TokenStream = rudder
        .get_functions()
        .into_iter()
        .map(|(name, function)| {
            let (return_type, parameters) = function.signature();
            let return_type = codegen_type(return_type);
            let parameters = codegen_parameters(parameters);
            let body = codegen_body(function);

            quote! {
                fn #name(#parameters) -> #return_type {
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

    quote! {
        #structs

        #fns
    }
}

fn codegen_parameters(parameters: Vec<Symbol>) -> TokenStream {
    let parameters = [quote!(state: &mut State)]
        .into_iter()
        .chain(parameters.iter().map(|sym| {
            let name = sym.name();
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
        width => panic!("width = {width}"),
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
            quote!([#element_type; #element_count])
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
            let name = format_ident!("{}", symbol.name().to_string());
            let typ = codegen_type(symbol.typ());

            if symbol.typ().is_unit() {
                quote! { let mut #name: () = (); }
            } else {
                quote! {
                    let mut #name: #typ = #typ::default();
                }
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

            quote! { #block_name => { #block_impl } }
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

fn get_ident(stmt: Statement) -> TokenStream {
    format_ident!("{}", stmt.name().to_string()).to_token_stream()
}

fn codegen_stmt(stmt: Statement) -> TokenStream {
    let stmt_name = format_ident!("{}", stmt.name().to_string());

    let value = match stmt.kind() {
        StatementKind::Constant { value, typ } => {
            match value {
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
            }
        }
        StatementKind::ReadVariable { symbol } => {
            let var = format_ident!("{}", symbol.name().to_string());
            quote! {#var}
        }
        StatementKind::WriteVariable { symbol, value } => {
            let var = format_ident!("{}", symbol.name().to_string());
            let value = get_ident(value);
            quote! {#var = #value}
        }
        StatementKind::ReadRegister { typ, offset } => {
            let offset = get_ident(offset);
            let typ = codegen_type(typ);
            quote!(unsafe {*(state.data.as_ptr().byte_offset(#offset as isize) as *const #typ)})
        }
        StatementKind::WriteRegister { .. } => quote!(todo!("write-reg")),
        StatementKind::ReadMemory { .. } => quote!(todo!("read-mem")),
        StatementKind::WriteMemory { .. } => quote!(todo!("write-mem")),
        StatementKind::ReadPc => quote!(todo!("read-pc")),
        StatementKind::WritePc { .. } => quote!(todo!("write-pc")),
        StatementKind::BinaryOperation { kind, lhs, rhs } => {
            let left = get_ident(lhs.clone());
            let right = get_ident(rhs.clone());

            let op = match kind {
                BinaryOperationKind::CmpEq => quote! { (#left) == (#right) },
                BinaryOperationKind::Add => quote! { (#left) + (#right) },
                BinaryOperationKind::Sub => quote! { (#left) - (#right) },
                BinaryOperationKind::Multiply => quote! { (#left) * (#right) },
                BinaryOperationKind::Divide => quote! { (#left) / (#right) },
                BinaryOperationKind::Modulo => quote! { (#left) % (#right) },
                BinaryOperationKind::And => quote! { (#left) & (#right) },
                BinaryOperationKind::Or => quote! { (#left) | (#right) },
                BinaryOperationKind::Xor => quote! { (#left) ^ (#right) },
                BinaryOperationKind::CmpNe => quote! { (#left) != (#right) },
                BinaryOperationKind::CmpLt => quote! { (#left) < (#right) },
                BinaryOperationKind::CmpLe => quote! { (#left) <= (#right) },
                BinaryOperationKind::CmpGt => quote! { (#left) > (#right) },
                BinaryOperationKind::CmpGe => quote! { (#left) >= (#right) },
            };

            quote! { (#op) }
        }
        StatementKind::UnaryOperation { kind, value } => {
            let value = get_ident(value);

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
            let value = get_ident(value);
            let amount = get_ident(amount);

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
            let ident = format_ident!("{}", target.name().to_string());
            let args = args.iter().map(|arg| {
                let arg = get_ident(arg.clone());
                quote! {#arg}
            });

            // todo: remove this
            if target.name().as_ref().starts_with("unimplemented_") {
                let msg = target.name().to_string();
                quote! { unimplemented!(#msg) }
            } else {
                quote! {
                    #ident(state, #(#args),*)
                }
            }
        }
        StatementKind::Cast { typ, value, .. } => {
            let source = value.typ();
            let target = typ;
            let ident = get_ident(value);

            if source == target {
                quote! {
                    ((#ident))
                }
            } else if target == Rc::new(Type::u1()) {
                // todo: is special casing booleans like this necessary?
                // todo: this is duplicated in decode
                quote! {
                    ((#ident) != 0)
                }
            } else {
                let target = codegen_type(target);

                quote! {
                    ((#ident) as #target)
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
            let condition = get_ident(condition);
            let true_target = format_ident!("BLOCK_{}", true_target.name().to_string());
            let false_target = format_ident!("BLOCK_{}", false_target.name().to_string());

            quote! {
                current_block = if #condition { #true_target } else { #false_target };
            }
        }
        StatementKind::PhiNode { .. } => quote!(todo!("phi")),
        StatementKind::Return { value } => match value {
            Some(value) => {
                let name = value.name();
                quote! {return #name;}
            }
            None => {
                quote! {return return_value;}
            }
        },
        StatementKind::Select { .. } => quote!(todo!("select")),
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => {
            let typ = codegen_type(value.typ());

            let value = get_ident(value);
            let start = get_ident(start);
            let length = get_ident(length);

            // todo: pre-cast length to u32

            quote! (
                (
                    (#value >> #start) &
                    ((1 as #typ).checked_shl(#length as u32).map(|x| x - 1).unwrap_or(!0))
                )
            )
        }
        StatementKind::BitInsert { .. } => quote!(todo!("bitins")),
        StatementKind::Trap => quote!(panic!("it's a trap")),
        StatementKind::ReadField { value, field } => {
            let value = get_ident(value);
            let field = format_ident!("_{field}");
            quote!(#value.#field)
        }
        StatementKind::MutateField {
            composite,
            field,
            value,
        } => {
            let composite = get_ident(composite);
            let field = format_ident!("_{field}");
            let value = get_ident(value);
            quote! {
                {
                    let mut local = #composite;
                    local.#field = #value;
                    local
                }
            }
        }
        StatementKind::ReadElement { value, index } => {
            let value = get_ident(value);
            let index = get_ident(index);
            // todo remove this cast, need "machine word" size in rudder?
            quote!(#value[(#index) as usize])
        }
        StatementKind::MutateElement {
            vector,
            value,
            index,
        } => {
            let vector = get_ident(vector);
            let index = get_ident(index);
            let value = get_ident(value);
            // todo remove this cast, need "machine word" size in rudder?
            quote! {
                {
                    #vector[(#index) as usize] = #value;
                    vector
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
                    let value = get_ident(statement.clone());
                    quote!(#field_name: #value,)
                })
                .collect::<TokenStream>();
            quote!(#typ { #fields })

            // struct { _0: foo, _1: bar }
        }
    };

    let msg = stmt.to_string();
    if stmt.has_value() {
        let typ = codegen_type(stmt.typ());
        quote! {
            let _ = #msg;
            let #stmt_name: #typ = #value;
        }
    } else {
        quote! {
            let _ = #msg;
            #value;
        }
    }
}

fn codegen_member(idx: usize) -> Ident {
    Ident::new(&format!("_{idx}"), Span::call_site())
}
