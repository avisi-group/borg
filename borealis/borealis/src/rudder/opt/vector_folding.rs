use crate::{
    rudder::{
        constant_value::ConstantValue,
        statement::{build_at, cast_at, BinaryOperationKind, Location, StatementKind},
        Block, Function, Type,
    },
    util::arena::{Arena, Ref},
};

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;
    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= run_on_block(f.block_arena_mut(), block);
    }

    changed
}

/// Replace vector access on registers and locals with adding to the indices and
/// offset respectively
fn run_on_block(arena: &mut Arena<Block>, block: Ref<Block>) -> bool {
    let mut did_change = false;

    for stmt in block.get(arena).statements() {
        // if we have a write reg of an assign element of a read reg
        // replace with single write reg to element
        if let StatementKind::WriteRegister {
            offset: write_offset,
            value: write_value,
        } = stmt.get(block.get(arena).arena()).kind().clone()
        {
            if let StatementKind::AssignElement {
                vector: assign_vector,
                value: assign_value,
                index: assign_index,
            } = write_value.get(block.get(arena).arena()).kind().clone()
            {
                if let StatementKind::ReadRegister {
                    typ: _read_type,
                    offset: read_offset,
                } = assign_vector.get(block.get(arena).arena()).kind().clone()
                {
                    // write-register
                    // offset = write_offset + index * element type width bytes
                    // value = assign_value

                    //assert_eq!(write_offset.kind(), read_offset.kind());

                    let vector_width = build_at(
                        block,
                        arena,
                        StatementKind::Constant {
                            typ: (Type::u16()),
                            value: ConstantValue::UnsignedInteger(
                                assign_value
                                    .get(block.get(arena).arena())
                                    .typ(block.get(arena).arena())
                                    .width_bytes()
                                    .try_into()
                                    .unwrap(),
                            ),
                        },
                        Location::Before(stmt),
                    );
                    let vector_offset = build_at(
                        block,
                        arena,
                        StatementKind::BinaryOperation {
                            kind: BinaryOperationKind::Multiply,
                            lhs: assign_index,
                            rhs: vector_width,
                        },
                        Location::Before(stmt),
                    );
                    let offset = build_at(
                        block,
                        arena,
                        StatementKind::BinaryOperation {
                            kind: BinaryOperationKind::Add,
                            lhs: write_offset,
                            rhs: vector_offset,
                        },
                        Location::Before(stmt),
                    );

                    // after inserting offset calculation, re-insert all the post-statements

                    // replace kind to make sure future uses aren't invalidated
                    // todo: actually this is a write, we can just delete it and build it again
                    stmt.get_mut(block.get_mut(arena).arena_mut()).replace_kind(
                        StatementKind::WriteRegister {
                            offset,
                            value: assign_value,
                        },
                    );

                    did_change = true;
                }
            }
        }

        // if we're reading an element of a vec
        // see if index is constant (check if the bundle is constant)
        // if vector is a register read, add index to offset
        // todo: if vector is a local variable read, add index to indices
        if let StatementKind::ReadElement { vector, index } =
            stmt.get(block.get(arena).arena()).kind().clone()
        {
            if let StatementKind::ReadRegister { offset, .. } =
                vector.get(block.get(arena).arena()).kind().clone()
            {
                let element_type = stmt
                    .get(block.get(arena).arena())
                    .typ(block.get(arena).arena());

                let index = cast_at(block, arena, index, Type::s64(), Location::Before(stmt));

                let offset = cast_at(block, arena, offset, Type::s64(), Location::Before(stmt));

                let typ_width = build_at(
                    block,
                    arena,
                    StatementKind::Constant {
                        typ: (Type::s64()),
                        value: ConstantValue::SignedInteger(
                            i64::try_from(element_type.width_bytes()).unwrap(),
                        ),
                    },
                    Location::Before(stmt),
                );

                let index_scaled = build_at(
                    block,
                    arena,
                    StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Multiply,
                        lhs: index,
                        rhs: typ_width,
                    },
                    Location::Before(stmt),
                );

                let new_offset = build_at(
                    block,
                    arena,
                    StatementKind::BinaryOperation {
                        kind: BinaryOperationKind::Add,
                        lhs: index_scaled,
                        rhs: offset,
                    },
                    Location::Before(stmt),
                );

                // after inserting offset calculation, re-insert all the post-statements

                // replace kind to make sure future uses aren't invalidated
                // todo: actually this is a write, we can just delete it and build it again
                stmt.get_mut(block.get_mut(arena).arena_mut()).replace_kind(
                    StatementKind::ReadRegister {
                        typ: element_type,
                        offset: new_offset,
                    },
                );

                did_change = true;
            }
        }
    }

    did_change
}
