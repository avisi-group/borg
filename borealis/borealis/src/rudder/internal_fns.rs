use {
    crate::rudder::model::{
        constant_value::ConstantValue,
        function::{Function, Symbol},
        statement::{build, BinaryOperationKind, CastOperationKind, ShiftOperationKind, Statement},
        types::Type,
    },
    once_cell::sync::Lazy,
};

pub static REPLICATE_BITS_BOREALIS_INTERNAL: Lazy<Function> = Lazy::new(|| {
    // // bits << (bits.len() * 0) | bits << (bits.len() * 1) | bits << (bits.len()
    // * 2) ...

    // for i in 0..count {
    //     acc <<= bits.len();
    //     acc |= bits;
    // }

    let bits_symbol = Symbol::new("bits".into(), Type::Bits);
    let count_symbol = Symbol::new("count".into(), Type::u64());
    let local_count_symbol = Symbol::new("local_count".into(), Type::u64());
    let result_symbol = Symbol::new("result".into(), Type::Bits);

    let mut function = Function::new(
        "replicate_bits_borealis_internal".into(),
        Type::Bits,
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
            Statement::Return { value: read_result },
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
                typ: (Type::u64()),
                value: ConstantValue::UnsignedInteger(0),
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
                typ: (Type::u64()),
                value: ConstantValue::UnsignedInteger(1),
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

        // get the length of bits, then cast from u8 to bundle
        let len = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::SizeOf {
                value: read_bits.clone(),
            },
        );

        let _8 = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::Constant {
                typ: (Type::u8()),
                value: ConstantValue::UnsignedInteger(8),
            },
        );

        let cast_len = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::Cast {
                kind: CastOperationKind::ZeroExtend,
                typ: (Type::u8()),
                value: len.clone(),
            },
        );

        let bundle_len = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::Cast {
                kind: CastOperationKind::Convert,
                typ: (Type::Bits),
                value: cast_len.clone(),
            },
        );

        // shift result
        let shift_result = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::ShiftOperation {
                kind: ShiftOperationKind::LogicalShiftLeft,
                value: read_result.clone(),
                amount: bundle_len.clone(),
            },
        );

        // or result with bits
        let or_result = build(
            shift_block_ref,
            function.arena_mut(),
            Statement::BinaryOperation {
                kind: BinaryOperationKind::Or,
                lhs: shift_result.clone(),
                rhs: read_bits.clone(),
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
                typ: (Type::u128()),
                value: ConstantValue::UnsignedInteger(0),
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

        let read_count_cast = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::Cast {
                kind: CastOperationKind::Truncate,
                typ: (Type::u16()),
                value: read_count.clone(),
            },
        );

        let bits_length_cast = build(
            entry_block_ref,
            function.arena_mut(),
            Statement::Cast {
                kind: CastOperationKind::Truncate,
                typ: (Type::u16()),
                value: bits_length.clone(),
            },
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
                length: length.clone(),
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
