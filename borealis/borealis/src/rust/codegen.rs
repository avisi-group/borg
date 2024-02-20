use std::{
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::rudder::{Block, Context, Function, PrimitiveType, Statement, Symbol, Type};
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use syn::token::Token;

pub fn codegen(rudder: Context) -> TokenStream {
    rudder.update_names();

    rudder
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
        .collect()
}

fn codegen_parameters(parameters: Vec<Symbol>) -> TokenStream {
    let parameters =
        [quote!(state: &mut AArch64CoreState)]
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
    if width < 2 {
        width
    } else if width < 9 {
        8
    } else if width < 17 {
        16
    } else if width < 33 {
        32
    } else {
        64
    }
}

fn codegen_type(typ: Rc<Type>) -> TokenStream {
    match &*typ {
        Type::Primitive(typ) => {
            let width = promote_width(typ.width());

            let rust_type = match typ.type_class() {
                crate::rudder::PrimitiveTypeClass::Void => return quote!(()),
                crate::rudder::PrimitiveTypeClass::Unit => return quote!(()),
                crate::rudder::PrimitiveTypeClass::UnsignedInteger => {
                    if width == 1 {
                        format_ident!("bool")
                    } else {
                        format_ident!("u{}", width)
                    }
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

            let hashed = format_ident!("CT_{:x}", hasher.finish());

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

fn get_name(stmt: Statement) -> TokenStream {
    match stmt.kind() {
        crate::rudder::StatementKind::Constant { typ, value } => {
            let width = typ.width();

            match value {
                crate::rudder::ConstantValue::UnsignedInteger(v) => {
                    let typ = format_ident!("u{}", width);
                    quote! {#v}
                }
                crate::rudder::ConstantValue::SignedInteger(v) => {
                    let typ = format_ident!("i{}", width);
                    quote! {#v}
                }
                crate::rudder::ConstantValue::FloatingPoint(v) => {
                    let typ = format_ident!("f{}", width);
                    quote! {#v}
                }
                crate::rudder::ConstantValue::Unit => quote!(()),
            }
        }
        _ => format_ident!("{}", stmt.name().to_string()).to_token_stream(),
    }
}

fn codegen_stmt(stmt: Statement) -> TokenStream {
    let stmt_name = format_ident!("{}", stmt.name().to_string());

    let value = match stmt.kind() {
        crate::rudder::StatementKind::Constant { typ, value } => match value {
            crate::rudder::ConstantValue::UnsignedInteger(v) => quote!(#v),
            crate::rudder::ConstantValue::SignedInteger(v) => quote!(#v),
            crate::rudder::ConstantValue::FloatingPoint(v) => quote!(#v),
            crate::rudder::ConstantValue::Unit => quote!(()),
        },
        crate::rudder::StatementKind::ReadVariable { symbol } => {
            let var = format_ident!("{}", symbol.name().to_string());
            quote! {#var}
        }
        crate::rudder::StatementKind::WriteVariable { symbol, value } => {
            let var = format_ident!("{}", symbol.name().to_string());
            let value = get_name(value);
            quote! {#var = #value}
        }
        crate::rudder::StatementKind::ReadRegister { typ, offset } => quote!(todo!("read-reg")),
        crate::rudder::StatementKind::WriteRegister { offset, value } => quote!(todo!("write-reg")),
        crate::rudder::StatementKind::ReadMemory { typ, offset } => quote!(todo!("read-mem")),
        crate::rudder::StatementKind::WriteMemory { offset, value } => quote!(todo!("write-mem")),
        crate::rudder::StatementKind::ReadPc => quote!(todo!("read-pc")),
        crate::rudder::StatementKind::WritePc { value } => quote!(todo!("write-pc")),
        crate::rudder::StatementKind::BinaryOperation { kind, lhs, rhs } => {
            let lhs = get_name(lhs);
            let rhs = get_name(rhs);

            match kind {
                crate::rudder::BinaryOperationKind::CmpEq => quote! {#lhs == #rhs},
                crate::rudder::BinaryOperationKind::Add => quote! {#lhs + #rhs},
                crate::rudder::BinaryOperationKind::Sub => quote! {#lhs - #rhs},
                crate::rudder::BinaryOperationKind::Multiply => quote! {#lhs * #rhs},
                crate::rudder::BinaryOperationKind::Divide => quote! {#lhs / #rhs},
                crate::rudder::BinaryOperationKind::Modulo => quote! {#lhs % #rhs},
                crate::rudder::BinaryOperationKind::And => quote! {#lhs & #rhs},
                crate::rudder::BinaryOperationKind::Or => quote! {#lhs | #rhs},
                crate::rudder::BinaryOperationKind::Xor => quote! {#lhs ^ #rhs},
                crate::rudder::BinaryOperationKind::CmpNe => quote! {#lhs != #rhs},
                crate::rudder::BinaryOperationKind::CmpLt => quote! {#lhs < #rhs},
                crate::rudder::BinaryOperationKind::CmpLe => quote! {#lhs <= #rhs},
                crate::rudder::BinaryOperationKind::CmpGt => quote! {#lhs > #rhs},
                crate::rudder::BinaryOperationKind::CmpGe => quote! {#lhs >= #rhs},
            }
        }
        crate::rudder::StatementKind::UnaryOperation { kind, value } => {
            let value = get_name(value);

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
            let value = get_name(value);
            let amount = get_name(amount);

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
            let target = format_ident!("{}", target.name().to_string());
            let args = args.iter().map(|arg| {
                let arg = get_name(arg.clone());
                quote! {#arg}
            });

            quote! {
                #target(#(#args),*)
            }
        }
        crate::rudder::StatementKind::Cast { kind, typ, value } => {
            let value = get_name(value);
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
            let condition = get_name(condition);
            let true_target = format_ident!("BLOCK_{}", true_target.name().to_string());
            let false_target = format_ident!("BLOCK_{}", false_target.name().to_string());

            quote! {
                current_block = if #condition { #true_target } else { #false_target };
            }
        }
        crate::rudder::StatementKind::PhiNode { members } => quote!(todo!("phi")),
        crate::rudder::StatementKind::Return { value } => match value {
            Some(value) => {
                let value = get_name(value);
                quote! {return #value;}
            }
            None => {
                quote! {return;}
            }
        },
        crate::rudder::StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => quote!(todo!("select")),
        crate::rudder::StatementKind::BitExtract {
            value,
            start,
            length,
        } => quote!(todo!("bitex")),
        crate::rudder::StatementKind::BitInsert {
            original_value,
            insert_value,
            start,
            length,
        } => quote!(todo!("bitins")),
        crate::rudder::StatementKind::Trap => quote!(panic!("it's a trap")),
    };

    let cmt: TokenStream = format!("///! {}", stmt).parse().unwrap();
    if stmt.has_value() {
        quote! {
            #cmt
            let #stmt_name = #value;
        }
    } else {
        quote! {
            #cmt
            #value;
        }
    }
}
