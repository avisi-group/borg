use crate::rudder::{
        constant_value::ConstantValue,
        statement::{BinaryOperationKind, StatementBuilder, StatementKind},
        Block, Function, Type,
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
    let mut did_change = false;

    for stmt in block.statements() {
        // if we have a write reg of an assign element of a read reg
        // replace with single write reg to element
        if let StatementKind::WriteRegister {
            offset: write_offset,
            value: write_value,
        } = stmt.kind()
        {
            if let StatementKind::AssignElement {
                vector: assign_vector,
                value: assign_value,
                index: assign_index,
            } = write_value.kind()
            {
                if let StatementKind::ReadRegister {
                    typ: _read_type,
                    offset: read_offset,
                } = assign_vector.kind()
                {
                    // write-register
                    // offset = write_offset + index * element type width bytes
                    // value = assign_value

                    assert_eq!(write_offset.kind(), read_offset.kind());

                    let mut builder = StatementBuilder::new(block.weak());
                    let vector_width = builder.build(StatementKind::Constant {
                        typ: (Type::u16()),
                        value: ConstantValue::UnsignedInteger(
                            assign_value.typ().width_bytes().try_into().unwrap(),
                        ),
                    });
                    let vector_offset = builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Multiply,
                        lhs: assign_index,
                        rhs: vector_width,
                    });
                    let offset = builder.build(StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: write_offset,
                        rhs: vector_offset,
                    });

                    for new_statement in builder.finish() {
                        block.insert_statement_before(&stmt, new_statement);
                    }

                    stmt.replace_kind(StatementKind::WriteRegister {
                        offset,
                        value: assign_value,
                    });
                    did_change = true;
                }
            }
        }

        // if we're reading an element of a vec
        // see if index is constant (check if the bundle is constant)
        // if vector is a register read, add index to offset
        // todo: if vector is a local variable read, add index to indices
        if let StatementKind::ReadElement { vector, index } = stmt.kind() {
            if let StatementKind::ReadRegister { offset, .. } = vector.kind() {
                let element_type = stmt.typ();
                let mut builder = StatementBuilder::new(block.weak());

                let index = builder.generate_cast(index, Type::s64());

                let offset = builder.generate_cast(offset, Type::s64());

                let typ_width = builder.build(StatementKind::Constant {
                    typ: (Type::s64()),
                    value: ConstantValue::SignedInteger(
                        i64::try_from(element_type.width_bytes()).unwrap(),
                    ),
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

                did_change = true;
            }
        }
    }

    did_change
}
