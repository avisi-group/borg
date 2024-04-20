use {
    crate::rudder::{
        BinaryOperationKind, Block, ConstantValue, Function, StatementBuilder, StatementKind, Type,
    },
    log::trace,
    std::rc::Rc,
};

pub fn run(f: Function) -> bool {
    let mut changed = false;
    for block in f.entry_block().iter() {
        changed |= run_on_block(&block);
    }

    changed
}

/// Replace vector access on registers and locals with adding to the indices and
/// offset respectively
fn run_on_block(block: &Block) -> bool {
    for stmt in block.statements() {
        // if we're reading an element of a vec
        // see if index is constant (check if the bundle is constant)
        // if vector is a register read, add index to offset
        // if vector is a local variable read, add index to indices
        if let StatementKind::ReadElement { vector, index } = stmt.kind() {}
    }

    false
}
