use {
    rayon::iter::ParallelIterator,
    sailrs::{
        jib_ast::{self, Definition, Instruction, InstructionAux, Type, Value, Vl},
        sail_ast::Location,
        types::ListVec,
    },
};

pub fn apply_fn_denylist<I: Iterator<Item = jib_ast::Definition>>(
    iter: I,
) -> impl Iterator<Item = jib_ast::Definition> {
    iter.map(|def| {
        if let Definition::Fundef(name, idk, arguments, body) = def {
            let body = if !DENYLIST.contains(&name.as_interned().as_ref()) {
                body
            } else {
                ListVec::from(vec![Instruction {
                    inner: InstructionAux::Throw(Value::Lit(Vl::Unit, Type::Unit)),
                    annot: (0, Location::Unknown),
                }])
            };

            Definition::Fundef(name, idk, arguments, body)
        } else {
            def
        }
    })
}

const DENYLIST: &[&'static str] = &[
    "integer_update_subrange",
    "PhysMemTagWrite",
    "PhysMemTagRead",
];
