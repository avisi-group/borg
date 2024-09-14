use {
    crate::{
        rudder::{
            statement::{Statement, StatementBuilder, StatementKind},
            Block, Function,
        },
        util::arena::Ref,
    },
    common::HashMap,
};

const INLINE_SIZE_THRESHOLD: usize = 5;

pub fn run(f: &mut Function) -> bool {
    do_block_inlining(f)
}

fn do_block_inlining(f: &Function) -> bool {
    let mut changed = false;

    for block in f.block_iter() {
        changed |= inline_target_block(f, block);
    }

    changed
}

fn clone_statement(
    builder: &mut StatementBuilder,
    template: &Statement,
    mapping: &HashMap<Statement, Statement>,
) -> Statement {
    match template.kind() {
        StatementKind::BinaryOperation { kind, lhs, rhs } => {
            builder.build(StatementKind::BinaryOperation {
                kind,
                lhs: mapping.get(&lhs).unwrap().clone(),
                rhs: mapping.get(&rhs).unwrap().clone(),
            })
        }
        StatementKind::Constant { typ, value } => {
            builder.build(StatementKind::Constant { typ, value })
        }
        StatementKind::ReadVariable { symbol } => {
            builder.build(StatementKind::ReadVariable { symbol })
        }
        StatementKind::WriteVariable { symbol, value } => {
            builder.build(StatementKind::WriteVariable {
                symbol,
                value: mapping.get(&value).unwrap().clone(),
            })
        }
        StatementKind::ReadRegister { typ, offset } => builder.build(StatementKind::ReadRegister {
            typ,
            offset: mapping.get(&offset).unwrap().clone(),
        }),
        StatementKind::WriteRegister { offset, value } => {
            builder.build(StatementKind::WriteRegister {
                offset: mapping.get(&offset).unwrap().clone(),
                value: mapping.get(&value).unwrap().clone(),
            })
        }
        StatementKind::ReadMemory { offset, size } => builder.build(StatementKind::ReadMemory {
            offset: mapping.get(&offset).unwrap().clone(),
            size: mapping.get(&size).unwrap().clone(),
        }),
        StatementKind::WriteMemory { offset, value } => builder.build(StatementKind::WriteMemory {
            offset: mapping.get(&offset).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
        }),
        StatementKind::ReadPc => builder.build(StatementKind::ReadPc),
        StatementKind::WritePc { value } => builder.build(StatementKind::WritePc {
            value: mapping.get(&value).unwrap().clone(),
        }),
        StatementKind::UnaryOperation { kind, value } => {
            builder.build(StatementKind::UnaryOperation {
                kind,
                value: mapping.get(&value).unwrap().clone(),
            })
        }
        StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => builder.build(StatementKind::ShiftOperation {
            kind,
            value: mapping.get(&value).unwrap().clone(),
            amount: mapping.get(&amount).unwrap().clone(),
        }),
        StatementKind::Call { target, args, tail } => {
            let args = args
                .iter()
                .map(|stmt| mapping.get(stmt).unwrap().clone())
                .collect();

            builder.build(StatementKind::Call { target, args, tail })
        }
        StatementKind::Cast { kind, typ, value } => builder.build(StatementKind::Cast {
            kind,
            typ: typ.clone(),
            value: mapping.get(&value).unwrap().clone(),
        }),
        StatementKind::BitsCast {
            kind,
            typ,
            value,
            length,
        } => builder.build(StatementKind::BitsCast {
            kind,
            typ: typ.clone(),
            value: mapping.get(&value).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        }),
        StatementKind::Jump { target } => builder.build(StatementKind::Jump { target }),
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => builder.build(StatementKind::Branch {
            condition: mapping.get(&condition).unwrap().clone(),
            true_target,
            false_target,
        }),
        StatementKind::PhiNode { .. } => todo!(),
        StatementKind::Return { value } => builder.build(StatementKind::Return {
            value: mapping.get(&value).unwrap().clone(),
        }),
        StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => builder.build(StatementKind::Select {
            condition: mapping.get(&condition).unwrap().clone(),
            true_value: mapping.get(&true_value).unwrap().clone(),
            false_value: mapping.get(&false_value).unwrap().clone(),
        }),
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => builder.build(StatementKind::BitExtract {
            value: mapping.get(&value).unwrap().clone(),
            start: mapping.get(&start).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        }),
        StatementKind::BitInsert {
            target,
            source,
            start,
            length,
        } => builder.build(StatementKind::BitInsert {
            target: mapping.get(&target).unwrap().clone(),
            source: mapping.get(&source).unwrap().clone(),
            start: mapping.get(&start).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        }),
        StatementKind::ReadElement { vector, index } => builder.build(StatementKind::ReadElement {
            vector: mapping.get(&vector).unwrap().clone(),
            index: mapping.get(&index).unwrap().clone(),
        }),
        StatementKind::AssignElement {
            vector,
            value,
            index,
        } => builder.build(StatementKind::AssignElement {
            vector: mapping.get(&vector).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
            index: mapping.get(&index).unwrap().clone(),
        }),
        StatementKind::Panic(stmt) => {
            builder.build(StatementKind::Panic(mapping.get(&stmt).unwrap().clone()))
        }

        StatementKind::Assert { condition } => builder.build(StatementKind::Assert {
            condition: mapping.get(&condition).unwrap().clone(),
        }),

        StatementKind::CreateBits { value, length } => builder.build(StatementKind::CreateBits {
            value: mapping.get(&value).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        }),
        StatementKind::SizeOf { value } => builder.build(StatementKind::SizeOf {
            value: mapping.get(&value).unwrap().clone(),
        }),
        StatementKind::MatchesUnion { value, variant } => {
            builder.build(StatementKind::MatchesUnion {
                value: mapping.get(&value).unwrap().clone(),
                variant,
            })
        }
        StatementKind::UnwrapUnion { value, variant } => {
            builder.build(StatementKind::UnwrapUnion {
                value: mapping.get(&value).unwrap().clone(),
                variant,
            })
        }

        StatementKind::Undefined => builder.build(StatementKind::Undefined),
        StatementKind::TupleAccess { index, source } => builder.build(StatementKind::TupleAccess {
            source: mapping.get(&source).unwrap().clone(),
            index,
        }),
        StatementKind::GetFlag { flag, operation } => builder.build(StatementKind::GetFlag {
            flag,
            operation: mapping.get(&operation).unwrap().clone(),
        }),
        StatementKind::CreateTuple(values) => builder.build(StatementKind::CreateTuple(
            values
                .iter()
                .map(|v| mapping.get(&v).unwrap())
                .cloned()
                .collect(),
        )),
    }
}

fn inline_target_block(f: &Function, source_block: Ref<Block>) -> bool {
    // if a block ends in a jump statement, and the target block is "small", inline
    // it.
    let terminator = source_block
        .get(f.block_arena())
        .terminator_statement()
        .unwrap();

    let StatementKind::Jump {
        target: target_block,
    } = terminator.kind()
    else {
        return false;
    };

    if target_block.get(f.block_arena()).size() > INLINE_SIZE_THRESHOLD {
        return false;
    }

    // kill the jump statement, copy target block statements in.
    source_block
        .get(f.block_arena())
        .kill_statement(&terminator);

    let mut builder = StatementBuilder::new(source_block);

    let mut mapping = HashMap::default();
    for stmt in target_block.get(f.block_arena()).statements() {
        let cloned_stmt = clone_statement(&mut builder, &stmt, &mapping);
        mapping.insert(stmt, cloned_stmt.clone());
    }

    source_block
        .get(f.block_arena())
        .extend_statements(builder.finish().into_iter());

    true
}
