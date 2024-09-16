use {
    crate::{
        rudder::{
            statement::{build, StatementInner, StatementKind},
            Block, Function,
        },
        util::arena::{Arena, Ref},
    },
    common::HashMap,
};

const INLINE_SIZE_THRESHOLD: usize = 5;

pub fn run(f: &mut Function) -> bool {
    let mut changed = false;

    for block in f.block_iter().collect::<Vec<_>>().into_iter() {
        changed |= inline_target_block(f, block);
    }

    changed
}

fn inline_target_block(f: &mut Function, source_block: Ref<Block>) -> bool {
    // if a block ends in a jump statement, and the target block is "small", inline
    // it.
    let terminator = source_block
        .get(f.block_arena())
        .terminator_statement()
        .unwrap();

    let StatementKind::Jump {
        target: target_block,
    } = terminator
        .get(&source_block.get(f.block_arena()).statement_arena)
        .kind()
        .clone()
    else {
        return false;
    };

    if target_block.get(f.block_arena()).size() > INLINE_SIZE_THRESHOLD {
        return false;
    }

    // kill the jump statement, copy target block statements in.

    let mut mapping = HashMap::default();
    for stmt in target_block.get(f.block_arena()).statements() {
        let cloned_stmt = clone_statement(source_block, f.block_arena_mut(), stmt, &mapping);
        mapping.insert(stmt, cloned_stmt.clone());
    }

    true
}

fn clone_statement(
    source_block: Ref<Block>,
    block_arena: &mut Arena<Block>,
    template: Ref<StatementInner>,
    mapping: &HashMap<Ref<StatementInner>, Ref<StatementInner>>,
) -> Ref<StatementInner> {
    match template
        .get(source_block.get(&block_arena).arena())
        .kind()
        .clone()
    {
        StatementKind::BinaryOperation { kind, lhs, rhs } => build(
            source_block,
            block_arena,
            StatementKind::BinaryOperation {
                kind,
                lhs: mapping.get(&lhs).unwrap().clone(),
                rhs: mapping.get(&rhs).unwrap().clone(),
            },
        ),
        StatementKind::Constant { typ, value } => build(
            source_block,
            block_arena,
            StatementKind::Constant { typ, value },
        ),
        StatementKind::ReadVariable { symbol } => build(
            source_block,
            block_arena,
            StatementKind::ReadVariable { symbol },
        ),
        StatementKind::WriteVariable { symbol, value } => build(
            source_block,
            block_arena,
            StatementKind::WriteVariable {
                symbol,
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::ReadRegister { typ, offset } => build(
            source_block,
            block_arena,
            StatementKind::ReadRegister {
                typ,
                offset: mapping.get(&offset).unwrap().clone(),
            },
        ),
        StatementKind::WriteRegister { offset, value } => build(
            source_block,
            block_arena,
            StatementKind::WriteRegister {
                offset: mapping.get(&offset).unwrap().clone(),
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::ReadMemory { offset, size } => build(
            source_block,
            block_arena,
            StatementKind::ReadMemory {
                offset: mapping.get(&offset).unwrap().clone(),
                size: mapping.get(&size).unwrap().clone(),
            },
        ),
        StatementKind::WriteMemory { offset, value } => build(
            source_block,
            block_arena,
            StatementKind::WriteMemory {
                offset: mapping.get(&offset).unwrap().clone(),
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::ReadPc => build(source_block, block_arena, StatementKind::ReadPc),
        StatementKind::WritePc { value } => build(
            source_block,
            block_arena,
            StatementKind::WritePc {
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::UnaryOperation { kind, value } => build(
            source_block,
            block_arena,
            StatementKind::UnaryOperation {
                kind,
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => build(
            source_block,
            block_arena,
            StatementKind::ShiftOperation {
                kind,
                value: mapping.get(&value).unwrap().clone(),
                amount: mapping.get(&amount).unwrap().clone(),
            },
        ),
        StatementKind::Call { target, args } => {
            let args = args
                .iter()
                .map(|stmt| mapping.get(stmt).unwrap().clone())
                .collect();

            build(
                source_block,
                block_arena,
                StatementKind::Call { target, args },
            )
        }
        StatementKind::Cast { kind, typ, value } => build(
            source_block,
            block_arena,
            StatementKind::Cast {
                kind,
                typ: typ.clone(),
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::BitsCast {
            kind,
            typ,
            value,
            length,
        } => build(
            source_block,
            block_arena,
            StatementKind::BitsCast {
                kind,
                typ: typ.clone(),
                value: mapping.get(&value).unwrap().clone(),
                length: mapping.get(&length).unwrap().clone(),
            },
        ),
        StatementKind::Jump { target } => {
            build(source_block, block_arena, StatementKind::Jump { target })
        }
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => build(
            source_block,
            block_arena,
            StatementKind::Branch {
                condition: mapping.get(&condition).unwrap().clone(),
                true_target,
                false_target,
            },
        ),
        StatementKind::PhiNode { .. } => todo!(),
        StatementKind::Return { value } => build(
            source_block,
            block_arena,
            StatementKind::Return {
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => build(
            source_block,
            block_arena,
            StatementKind::Select {
                condition: mapping.get(&condition).unwrap().clone(),
                true_value: mapping.get(&true_value).unwrap().clone(),
                false_value: mapping.get(&false_value).unwrap().clone(),
            },
        ),
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => build(
            source_block,
            block_arena,
            StatementKind::BitExtract {
                value: mapping.get(&value).unwrap().clone(),
                start: mapping.get(&start).unwrap().clone(),
                length: mapping.get(&length).unwrap().clone(),
            },
        ),
        StatementKind::BitInsert {
            target,
            source,
            start,
            length,
        } => build(
            source_block,
            block_arena,
            StatementKind::BitInsert {
                target: mapping.get(&target).unwrap().clone(),
                source: mapping.get(&source).unwrap().clone(),
                start: mapping.get(&start).unwrap().clone(),
                length: mapping.get(&length).unwrap().clone(),
            },
        ),
        StatementKind::ReadElement { vector, index } => build(
            source_block,
            block_arena,
            StatementKind::ReadElement {
                vector: mapping.get(&vector).unwrap().clone(),
                index: mapping.get(&index).unwrap().clone(),
            },
        ),
        StatementKind::AssignElement {
            vector,
            value,
            index,
        } => build(
            source_block,
            block_arena,
            StatementKind::AssignElement {
                vector: mapping.get(&vector).unwrap().clone(),
                value: mapping.get(&value).unwrap().clone(),
                index: mapping.get(&index).unwrap().clone(),
            },
        ),
        StatementKind::Panic(stmt) => build(
            source_block,
            block_arena,
            StatementKind::Panic(mapping.get(&stmt).unwrap().clone()),
        ),

        StatementKind::Assert { condition } => build(
            source_block,
            block_arena,
            StatementKind::Assert {
                condition: mapping.get(&condition).unwrap().clone(),
            },
        ),

        StatementKind::CreateBits { value, length } => build(
            source_block,
            block_arena,
            StatementKind::CreateBits {
                value: mapping.get(&value).unwrap().clone(),
                length: mapping.get(&length).unwrap().clone(),
            },
        ),
        StatementKind::SizeOf { value } => build(
            source_block,
            block_arena,
            StatementKind::SizeOf {
                value: mapping.get(&value).unwrap().clone(),
            },
        ),
        StatementKind::MatchesUnion { value, variant } => build(
            source_block,
            block_arena,
            StatementKind::MatchesUnion {
                value: mapping.get(&value).unwrap().clone(),
                variant,
            },
        ),
        StatementKind::UnwrapUnion { value, variant } => build(
            source_block,
            block_arena,
            StatementKind::UnwrapUnion {
                value: mapping.get(&value).unwrap().clone(),
                variant,
            },
        ),

        StatementKind::Undefined => build(source_block, block_arena, StatementKind::Undefined),
        StatementKind::TupleAccess { index, source } => build(
            source_block,
            block_arena,
            StatementKind::TupleAccess {
                source: mapping.get(&source).unwrap().clone(),
                index,
            },
        ),
        StatementKind::GetFlag { flag, operation } => build(
            source_block,
            block_arena,
            StatementKind::GetFlag {
                flag,
                operation: mapping.get(&operation).unwrap().clone(),
            },
        ),
        StatementKind::CreateTuple(values) => build(
            source_block,
            block_arena,
            StatementKind::CreateTuple(
                values
                    .iter()
                    .map(|v| mapping.get(&v).unwrap())
                    .cloned()
                    .collect(),
            ),
        ),
    }
}
