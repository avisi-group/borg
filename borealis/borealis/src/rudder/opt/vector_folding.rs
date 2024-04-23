use {
    crate::rudder::{
        BinaryOperationKind, Block, ConstantValue, Function, StatementBuilder, StatementKind, Type,
    },
    std::sync::Arc,
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
        // todo: if vector is a local variable read, add index to indices
        if let StatementKind::ReadElement { vector, index } = stmt.kind() {
            if let StatementKind::ReadRegister { offset, .. } = vector.kind() {
                let element_type = stmt.typ();
                let mut builder = StatementBuilder::new(block.weak());

                let index = {
                    let index = if index.typ().is_bundle() {
                        builder.build(StatementKind::UnbundleValue { bundle: index })
                    } else {
                        index
                    };
                    builder.generate_cast(index, Arc::new(Type::s64()))
                };

                let offset = {
                    let offset = if offset.typ().is_bundle() {
                        builder.build(StatementKind::UnbundleValue { bundle: offset })
                    } else {
                        offset
                    };
                    builder.generate_cast(offset, Arc::new(Type::s64()))
                };

                let typ_width = builder.build(StatementKind::Constant {
                    typ: Arc::new(Type::s64()),
                    value: ConstantValue::SignedInteger(element_type.width_bytes() as isize),
                });

                let index_scaled = builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Multiply,
                    lhs: index,
                    rhs: typ_width,
                });

                let new_offset = builder.build(StatementKind::BinaryOperation {
                    kind: BinaryOperationKind::Add,
                    lhs: index_scaled,
                    rhs: offset,
                });

                for new_statement in builder.finish() {
                    block.insert_statement_before(&stmt, new_statement);
                }

                stmt.replace_kind(StatementKind::ReadRegister {
                    typ: element_type,
                    offset: new_offset,
                });
            }
        }
    }

    false
}
