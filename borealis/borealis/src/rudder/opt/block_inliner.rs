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
        changed |= run_inliner_block(f, block);
    }

    changed
}

fn run_inliner_block(f: &mut Function, source_block: Ref<Block>) -> bool {
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
    source_block
        .get_mut(f.block_arena_mut())
        .kill_statement(terminator);

    let mut mapping = HashMap::default();

    for target_statement in target_block.get(f.block_arena()).statements() {
        let source_statement = import_statement(
            source_block,
            target_block,
            f.block_arena_mut(),
            target_statement,
            &mapping,
        );

        mapping.insert(target_statement, source_statement);
    }

    true
}

fn import_statement(
    source_block: Ref<Block>,
    target_block: Ref<Block>,
    block_arena: &mut Arena<Block>,
    target_statement: Ref<StatementInner>,
    mapping: &HashMap<Ref<StatementInner>, Ref<StatementInner>>,
) -> Ref<StatementInner> {
    let mapped_kind = match target_statement
        .get(target_block.get(&block_arena).arena())
        .kind()
        .clone()
    {
        StatementKind::BinaryOperation { kind, lhs, rhs } => StatementKind::BinaryOperation {
            kind,
            lhs: mapping.get(&lhs).unwrap().clone(),
            rhs: mapping.get(&rhs).unwrap().clone(),
        },
        StatementKind::Constant { typ, value } => StatementKind::Constant { typ, value },
        StatementKind::ReadVariable { symbol } => StatementKind::ReadVariable { symbol },
        StatementKind::WriteVariable { symbol, value } => StatementKind::WriteVariable {
            symbol,
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ReadRegister { typ, offset } => StatementKind::ReadRegister {
            typ,
            offset: mapping.get(&offset).unwrap().clone(),
        },
        StatementKind::WriteRegister { offset, value } => StatementKind::WriteRegister {
            offset: mapping.get(&offset).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ReadMemory { offset, size } => StatementKind::ReadMemory {
            offset: mapping.get(&offset).unwrap().clone(),
            size: mapping.get(&size).unwrap().clone(),
        },
        StatementKind::WriteMemory { offset, value } => StatementKind::WriteMemory {
            offset: mapping.get(&offset).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ReadPc => StatementKind::ReadPc,
        StatementKind::WritePc { value } => StatementKind::WritePc {
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::UnaryOperation { kind, value } => StatementKind::UnaryOperation {
            kind,
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::ShiftOperation {
            kind,
            value,
            amount,
        } => StatementKind::ShiftOperation {
            kind,
            value: mapping.get(&value).unwrap().clone(),
            amount: mapping.get(&amount).unwrap().clone(),
        },
        StatementKind::Call { target, args } => {
            let args = args
                .iter()
                .map(|stmt| mapping.get(stmt).unwrap().clone())
                .collect();

            StatementKind::Call { target, args }
        }
        StatementKind::Cast { kind, typ, value } => StatementKind::Cast {
            kind,
            typ: typ.clone(),
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::BitsCast {
            kind,
            typ,
            value,
            length,
        } => StatementKind::BitsCast {
            kind,
            typ: typ.clone(),
            value: mapping.get(&value).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::Jump { target } => StatementKind::Jump { target },
        StatementKind::Branch {
            condition,
            true_target,
            false_target,
        } => StatementKind::Branch {
            condition: mapping.get(&condition).unwrap().clone(),
            true_target,
            false_target,
        },
        StatementKind::PhiNode { .. } => todo!(),
        StatementKind::Return { value } => StatementKind::Return {
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::Select {
            condition,
            true_value,
            false_value,
        } => StatementKind::Select {
            condition: mapping.get(&condition).unwrap().clone(),
            true_value: mapping.get(&true_value).unwrap().clone(),
            false_value: mapping.get(&false_value).unwrap().clone(),
        },
        StatementKind::BitExtract {
            value,
            start,
            length,
        } => StatementKind::BitExtract {
            value: mapping.get(&value).unwrap().clone(),
            start: mapping.get(&start).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::BitInsert {
            target,
            source,
            start,
            length,
        } => StatementKind::BitInsert {
            target: mapping.get(&target).unwrap().clone(),
            source: mapping.get(&source).unwrap().clone(),
            start: mapping.get(&start).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::ReadElement { vector, index } => StatementKind::ReadElement {
            vector: mapping.get(&vector).unwrap().clone(),
            index: mapping.get(&index).unwrap().clone(),
        },
        StatementKind::AssignElement {
            vector,
            value,
            index,
        } => StatementKind::AssignElement {
            vector: mapping.get(&vector).unwrap().clone(),
            value: mapping.get(&value).unwrap().clone(),
            index: mapping.get(&index).unwrap().clone(),
        },
        StatementKind::Panic(stmt) => StatementKind::Panic(mapping.get(&stmt).unwrap().clone()),

        StatementKind::Assert { condition } => StatementKind::Assert {
            condition: mapping.get(&condition).unwrap().clone(),
        },

        StatementKind::CreateBits { value, length } => StatementKind::CreateBits {
            value: mapping.get(&value).unwrap().clone(),
            length: mapping.get(&length).unwrap().clone(),
        },
        StatementKind::SizeOf { value } => StatementKind::SizeOf {
            value: mapping.get(&value).unwrap().clone(),
        },
        StatementKind::MatchesUnion { value, variant } => StatementKind::MatchesUnion {
            value: mapping.get(&value).unwrap().clone(),
            variant,
        },
        StatementKind::UnwrapUnion { value, variant } => StatementKind::UnwrapUnion {
            value: mapping.get(&value).unwrap().clone(),
            variant,
        },

        StatementKind::Undefined => StatementKind::Undefined,
        StatementKind::TupleAccess { index, source } => StatementKind::TupleAccess {
            source: mapping.get(&source).unwrap().clone(),
            index,
        },
        StatementKind::GetFlag { flag, operation } => StatementKind::GetFlag {
            flag,
            operation: mapping.get(&operation).unwrap().clone(),
        },
        StatementKind::CreateTuple(values) => StatementKind::CreateTuple(
            values
                .iter()
                .map(|v| mapping.get(&v).unwrap())
                .cloned()
                .collect(),
        ),
    };

    build(source_block, block_arena, mapped_kind)
}
