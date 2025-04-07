use {
    crate::boom::{self, control_flow::ControlFlowBlock},
    common::{
        hashmap::HashMap,
        intern::InternedString,
        rudder::{
            constant::Constant,
            function::{Function, Symbol},
            statement::{BinaryOperationKind, ShiftOperationKind, Statement, build, cast},
            types::Type,
        },
    },
    once_cell::sync::Lazy,
    sailrs::shared::Shared,
};

pub fn insert_stub(
    functions: &mut HashMap<InternedString, (Function, boom::FunctionDefinition)>,
    f: &Function,
) {
    functions.insert(
        f.name(),
        (
            // have to make a new function here or `build_functions` will overwrite it
            Function::new(f.name(), f.return_type(), f.parameters()),
            boom::FunctionDefinition {
                signature: boom::FunctionSignature {
                    name: f.name(),
                    parameters: Shared::new(vec![]),
                    return_type: None,
                },
                entry_block: ControlFlowBlock::new(),
            },
        ),
    );
}

pub static REPLICATE_BITS_BOREALIS_INTERNAL: Lazy<Function> = Lazy::new(|| {
    // // bits << (bits.len() * 0) | bits << (bits.len() * 1) | bits << (bits.len()
    // * 2) ...

    // for i in 0..count {
    //     acc <<= bits.len();
    //     acc |= bits;
    // }

    let bits_symbol = Symbol::new("bits".into(), Type::Bits);
    let count_symbol = Symbol::new("count".into(), Type::s64());
    let local_count_symbol = Symbol::new("local_count".into(), Type::s64());
    let result_symbol = Symbol::new("result".into(), Type::Bits);

    let mut function = Function::new(
        "replicate_bits_borealis_internal".into(),
        Some(Type::Bits),
        vec![bits_symbol.clone(), count_symbol.clone()],
    );
    function.add_local_variable(result_symbol.clone());
    function.add_local_variable(local_count_symbol.clone());

    let end_block_ref = {
        let end_block_ref = function.new_block();

        let read_result = build(
            end_block_ref,
            function.arena_mut(),
            Statement::ReadVariable {
                symbol: result_symbol.clone(),
            },
        );

        build(
            end_block_ref,
            function.arena_mut(),
            Statement::Return {
                value: Some(read_result),
            },
        );

        end_block_ref
    };

    // cyclic so need both created here
    let check_block_ref = function.new_block();
    let shift_block_ref = function.new_block();

    // check block if local_count == 0
    {
        let _0 = build(
            check_block_ref,
            function.arena_mut(),
            Statement::Constant {
                typ: Type::s64(),
                value: Constant::new_signed(0, 64),
            },
        );

        let read_count = build(
            check_block_ref,
            function.arena_mut(),
            Statement::ReadVariable {
                symbol: local_count_symbol.clone(),
            },
        );

        let count_is_zero = build(
            check_block_ref,
            function.arena_mut(),
            Statement::BinaryOperation {
                kind: BinaryOperationKind::CompareEqual,
                lhs: read_count.clone(),
                rhs: _0.clone(),
            },
        );

        build(
            check_block_ref,
            function.arena_mut(),
            Statement::Branch {
                condition: count_is_zero,
                true_target: end_block_ref,
                false_target: shift_block_ref,
            },
        );
    }

    // decrement count
    {
        let read_count = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::ReadVariable {
                symbol: local_count_symbol.clone(),
            },
        );

        let _1 = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::Constant {
                typ: Type::s64(),
                value: Constant::new_signed(1, 64),
            },
        );

        let decrement = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::BinaryOperation {
                kind: BinaryOperationKind::Sub,
                lhs: read_count.clone(),
                rhs: _1.clone(),
            },
        );

        build(
            shift_block_ref,
            function.arena_mut(),
            Statement::WriteVariable {
                symbol: local_count_symbol.clone(),
                value: decrement.clone(),
            },
        );

        // read result and bits variables
        let read_result = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::ReadVariable {
                symbol: result_symbol.clone(),
            },
        );

        let read_bits = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::ReadVariable {
                symbol: bits_symbol.clone(),
            },
        );

        let result_type = read_result
            .get(shift_block_ref.get(function.arena()).arena())
            .typ(shift_block_ref.get(function.arena()).arena())
            .unwrap();
        let cast_read_bits = cast(
            shift_block_ref,
            function.arena_mut(),
            read_bits,
            result_type,
        );

        // get the length of bits, then cast from u8 to bundle
        let len = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::SizeOf {
                value: read_bits.clone(),
            },
        );

        // shift result
        let shift_result = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::ShiftOperation {
                kind: ShiftOperationKind::LogicalShiftLeft,
                value: read_result.clone(),
                amount: len.clone(),
            },
        );

        // or result with bits
        let or_result = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::BinaryOperation {
                kind: BinaryOperationKind::Or,
                lhs: shift_result.clone(),
                rhs: cast_read_bits.clone(),
            },
        );

        // write result
        build(
            shift_block_ref,
            function.arena_mut(),
            Statement::WriteVariable {
                symbol: result_symbol.clone(),
                value: or_result.clone(),
            },
        );

        // jump
        build(
            shift_block_ref,
            function.arena_mut(),
            Statement::Jump {
                target: check_block_ref,
            },
        );
    }

    let entry_block = {
        let entry_block_ref = function.new_block();

        // copy count to count_local
        // jump to check block
        let read_count = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::ReadVariable {
                symbol: count_symbol.clone(),
            },
        );

        build(
            entry_block_ref,
            function.arena_mut(),
            Statement::WriteVariable {
                symbol: local_count_symbol.clone(),
                value: read_count.clone(),
            },
        );

        let zero = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::Constant {
                typ: (Type::u64()),
                value: Constant::new_unsigned(0, 64),
            },
        );

        let bits = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::ReadVariable {
                symbol: bits_symbol.clone(),
            },
        );

        let bits_length = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::SizeOf {
                value: bits.clone(),
            },
        );

        let read_count_cast = cast(
            entry_block_ref,
            function.arena_mut(),
            read_count.clone(),
            Type::s16(),
        );

        let bits_length_cast = cast(
            entry_block_ref,
            function.arena_mut(),
            bits_length.clone(),
            Type::s16(),
        );

        let length = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::BinaryOperation {
                kind: BinaryOperationKind::Multiply,
                lhs: read_count_cast.clone(),
                rhs: bits_length_cast.clone(),
            },
        );

        let result_bits = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::CreateBits {
                value: zero.clone(),
                width: length.clone(),
            },
        );

        // write result
        build(
            entry_block_ref,
            function.arena_mut(),
            Statement::WriteVariable {
                symbol: result_symbol.clone(),
                value: result_bits.clone(),
            },
        );

        build(
            entry_block_ref,
            function.arena_mut(),
            Statement::Jump {
                target: check_block_ref,
            },
        );

        entry_block_ref
    };

    function.set_entry_block(entry_block);

    function
});
