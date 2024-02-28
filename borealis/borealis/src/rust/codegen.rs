use {
    crate::rudder::{Block, Context, Function, Statement, Symbol, Type},
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
                #[derive(Default)]
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
            let width = promote_width(typ.width());

            let rust_type = match typ.type_class() {
                crate::rudder::PrimitiveTypeClass::Void => return quote!(()),
                crate::rudder::PrimitiveTypeClass::Unit => return quote!(()),
                crate::rudder::PrimitiveTypeClass::UnsignedInteger => {
                    format_ident!("u{}", width)
                }
                crate::rudder::PrimitiveTypeClass::SignedInteger => {
                    format_ident!("i{}", width)
                }
                crate::rudder::PrimitiveTypeClass::FloatingPoint => {
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
            let idx = idx as u32;

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
        crate::rudder::StatementKind::Constant { value, .. } => {
            match value {
                // todo: do not emit type info here
                crate::rudder::ConstantValue::UnsignedInteger(v) => {
                    let v = Literal::usize_unsuffixed(v);
                    quote!(#v)
                }
                crate::rudder::ConstantValue::SignedInteger(v) => {
                    let v = Literal::isize_unsuffixed(v);
                    quote!(#v)
                }
                crate::rudder::ConstantValue::FloatingPoint(v) => {
                    let v = Literal::f64_unsuffixed(v);
                    quote!(#v)
                }

                crate::rudder::ConstantValue::Unit => quote!(()),
            }
        }
        crate::rudder::StatementKind::ReadVariable { symbol, member } => {
            let var = format_ident!("{}", symbol.name().to_string());
            let member = member.map(codegen_member).map(|ident| quote!(.#ident));
            quote! {#var #member}
        }
        crate::rudder::StatementKind::WriteVariable {
            symbol,
            member,
            value,
        } => {
            let typ = codegen_type(symbol.typ());
            let var = format_ident!("{}", symbol.name().to_string());
            let member = member.map(codegen_member).map(|ident| quote!(.#ident));
            let value = get_ident(value);
            quote! {#var #member = #value as #typ}
        }
        crate::rudder::StatementKind::ReadRegister { .. } => quote!(todo!("read-reg")),
        crate::rudder::StatementKind::WriteRegister { .. } => quote!(todo!("write-reg")),
        crate::rudder::StatementKind::ReadMemory { .. } => quote!(todo!("read-mem")),
        crate::rudder::StatementKind::WriteMemory { .. } => quote!(todo!("write-mem")),
        crate::rudder::StatementKind::ReadPc => quote!(todo!("read-pc")),
        crate::rudder::StatementKind::WritePc { .. } => quote!(todo!("write-pc")),
        crate::rudder::StatementKind::BinaryOperation { kind, lhs, rhs } => {
            let left = get_ident(lhs.clone());
            let right = get_ident(rhs.clone());

            let cmp_type = if lhs.get_type().width() > rhs.get_type().width() {
                lhs.get_type()
            } else {
                rhs.get_type()
            };

            let rs_type = codegen_type(cmp_type);
            let target_type = codegen_type(stmt.get_type());

            let left = quote! { #rs_type::from(#left) };
            let right = quote! { #rs_type::from(#right) };

            let op = match kind {
                crate::rudder::BinaryOperationKind::CmpEq => quote! { #left == #right },
                crate::rudder::BinaryOperationKind::Add => quote! {#left + #right},
                crate::rudder::BinaryOperationKind::Sub => quote! {#left - #right},
                crate::rudder::BinaryOperationKind::Multiply => quote! {#left * #right},
                crate::rudder::BinaryOperationKind::Divide => quote! {#left / #right},
                crate::rudder::BinaryOperationKind::Modulo => quote! {#left % #right},
                crate::rudder::BinaryOperationKind::And => quote! {#left & #right},
                crate::rudder::BinaryOperationKind::Or => quote! {#left | #right},
                crate::rudder::BinaryOperationKind::Xor => quote! {#left ^ #right},
                crate::rudder::BinaryOperationKind::CmpNe => quote! {#left != #right},
                crate::rudder::BinaryOperationKind::CmpLt => quote! {#left < #right},
                crate::rudder::BinaryOperationKind::CmpLe => quote! {#left <= #right},
                crate::rudder::BinaryOperationKind::CmpGt => quote! {#left > #right},
                crate::rudder::BinaryOperationKind::CmpGe => quote! {#left >= #right},
            };

            // todo: don't blindly cast
            quote! { (#op) as #target_type }
        }
        crate::rudder::StatementKind::UnaryOperation { kind, value } => {
            let value = get_ident(value);

            match kind {
                crate::rudder::UnaryOperationKind::Not => quote! {!#value},
                crate::rudder::UnaryOperationKind::Negate => quote! {-#value},
                crate::rudder::UnaryOperationKind::Complement => quote! {!#value},
            }
        }
        crate::rudder::StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => {
            let value = get_ident(value);
            let amount = get_ident(amount);

            match kind {
                crate::rudder::ShiftOperationKind::LogicalShiftLeft => quote! {#value << #amount},
                crate::rudder::ShiftOperationKind::LogicalShiftRight => quote! {#value >> #amount},
                crate::rudder::ShiftOperationKind::ArithmeticShiftRight => {
                    quote! {#value >> #amount}
                }
                crate::rudder::ShiftOperationKind::RotateRight => todo!(),
                crate::rudder::ShiftOperationKind::RotateLeft => todo!(),
            }
        }
        crate::rudder::StatementKind::Call { target, args } => {
            // // todo: remove me

            let ident = format_ident!("{}", target.name().to_string());
            let args = args.iter().map(|arg| {
                let arg = get_ident(arg.clone());
                quote! {#arg}
            });

            if target.name().as_ref().starts_with("unimplemented_") {
                let msg = target.name().to_string();
                quote! { unimplemented!(#msg) }
            } else {
                quote! {
                    #ident(state, #(#args),*)
                }
            }
        }
        crate::rudder::StatementKind::Cast { typ, value, .. } => {
            let value = get_ident(value);
            let typ = codegen_type(typ);

            quote! {
                #value as #typ
            }
        }
        crate::rudder::StatementKind::Jump { target } => {
            let target = format_ident!("BLOCK_{}", target.name().to_string());
            quote! {
                current_block = #target;
            }
        }
        crate::rudder::StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => {
            let condition = get_ident(condition);
            let true_target = format_ident!("BLOCK_{}", true_target.name().to_string());
            let false_target = format_ident!("BLOCK_{}", false_target.name().to_string());

            quote! {
                current_block = if #condition != 0 { #true_target } else { #false_target };
            }
        }
        crate::rudder::StatementKind::PhiNode { .. } => quote!(todo!("phi")),
        crate::rudder::StatementKind::Return { value } => match value {
            Some(value) => {
                let name = value.name();
                quote! {return #name;}
            }
            None => {
                quote! {return return_value;}
            }
        },
        crate::rudder::StatementKind::Select { .. } => quote!(todo!("select")),
        crate::rudder::StatementKind::BitExtract { .. } => quote!(todo!("bitex")),
        crate::rudder::StatementKind::BitInsert { .. } => quote!(todo!("bitins")),
        crate::rudder::StatementKind::Trap => quote!(panic!("it's a trap")),
    };

    let msg = stmt.to_string();
    if stmt.has_value() {
        let typ = codegen_type(stmt.get_type());
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
