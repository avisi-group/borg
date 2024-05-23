use rayon::iter::ParallelIterator;
use sailrs::{
    jib_ast::{self, Definition, Instruction, InstructionAux, Type, Value, Vl},
    sail_ast::Location,
    types::ListVec,
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
    "ExecuteA64", // unknown ident `exn`
    "ExecuteA32",
    "ExecuteT32__1", // unknown ident `exn`
    "__TryDecodeExecute",
    "__DecodeExecute",
    "__InstructionExecute", // unknown ident `exn`
    "__TopLevel",
    "sail_mem_read",
    "sail_mem_write",
    "read_request",
    "write_request",
    "take_exception",
    "gic_readonly",
    "__ReadUART",
    "__WriteUART",
    "PhysMemTagWrite",
    "PhysMemTagRead",
    "__EndCycle",
    "__ListConfig",
    "PendSErrorInterrupt",
    "step_model",
    "main",
];
