use {
    crate::{
        boom,
        rudder::{self, Context, Function, FunctionKind},
    },
    common::intern::InternedString,
    proc_macro2::TokenStream,
    quote::ToTokens,
    std::{
        cell::RefCell,
        collections::{HashMap, HashSet, LinkedList},
        fmt::Display,
        hash::{Hash, Hasher},
        rc::Rc,
    },
};

pub fn from_boom(ast: &boom::Ast) -> Context {
    let mut fns =
        HashMap::<InternedString, (FunctionKind, Function, boom::FunctionDefinition)>::new();

    // need all functions with signatures before building
    for (boom_function_name, boom_function) in &ast.functions {
        fns.insert(
            boom_function_name.clone(),
            (
                FunctionKind::Execute,
                Function::new(
                    build_type(boom_function.signature.return_type.clone()),
                    boom_function
                        .signature
                        .parameters
                        .borrow()
                        .iter()
                        .map(|boom::Parameter { typ, .. }| typ.clone())
                        .map(build_type)
                        .collect(),
                ),
                boom_function.clone(),
            ),
        );
    }

    let test = fns
        .get(&InternedString::new(
            "integer_arithmetic_addsub_carry_decode",
        ))
        .unwrap();

    build_fn(&test.1, &test.2);

    Context {
        fns: fns
            .into_iter()
            .filter_map(|(name, (kind, func, _))| {
                if name == "integer_arithmetic_addsub_carry_decode".into() {
                    Some((name, (kind, func)))
                } else {
                    None
                }
            })
            .collect(),
    }
}

fn build_fn(rudder_fn: &rudder::Function, boom_fn: &boom::FunctionDefinition) {

    // build blocks
}

fn build_type(typ: Rc<RefCell<boom::Type>>) -> rudder::Type {
    match &*typ.borrow() {
        boom::Type::Unit => rudder::Type::void(),
        boom::Type::String => todo!(),
        boom::Type::Bool | boom::Type::Bit => rudder::Type::u1(),
        boom::Type::Real => rudder::Type::f32(),
        boom::Type::Float => rudder::Type::f64(),
        boom::Type::Int { signed, size } => {
            let tc = match signed {
                true => rudder::PrimitiveTypeClass::SignedInteger,
                false => rudder::PrimitiveTypeClass::UnsignedInteger,
            };

            let element_width = match size {
                boom::Size::Static(size) => *size,
                boom::Size::Runtime(_) | boom::Size::Unknown => 64, // todo blame tom when this inevitably breaks
            };

            rudder::Type::new_primitive(tc, element_width)
        }
        boom::Type::Constant(_) => todo!(),
        boom::Type::Enum { .. } => rudder::Type::u32(),
        boom::Type::Union { name, fields } | boom::Type::Struct { name, fields } => todo!(),
        boom::Type::List { element_type } => todo!(),
        boom::Type::Vector { element_type } => todo!(),
        boom::Type::FixedVector {
            length,
            element_type,
        } => todo!(),
        boom::Type::Reference(_) => todo!(),
    }
}
