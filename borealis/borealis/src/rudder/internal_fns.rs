use {
    crate::rudder::{
        constant_value::ConstantValue,
        statement::{
            BinaryOperationKind, CastOperationKind, ShiftOperationKind, StatementBuilder,
            StatementKind,
        },
        Function, Symbol, Type,
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

    let bits_symbol = Symbol {
        name: "bits".into(),

        typ: (Type::Bits),
    };
    let count_symbol = Symbol {
        name: "count".into(),

        typ: (Type::u64()),
    };

    let local_count_symbol = Symbol {
        name: "local_count".into(),
        typ: (Type::u64()),
    };
    let result_symbol = Symbol {
        name: "result".into(),
        typ: (Type::Bits),
    };

    let mut function = Function::new(
        "replicate_bits_borealis_internal".into(),
        Type::Bits,
        vec![bits_symbol.clone(), count_symbol.clone()],
    );
    function.add_local_variable(result_symbol.clone());
    function.add_local_variable(local_count_symbol.clone());

    let end_block_ref = {
        let end_block_ref = function.new_block();
        let mut builder = StatementBuilder::new(end_block_ref);

        let read_result = builder.build(StatementKind::ReadVariable {
            symbol: result_symbol.clone(),
        });

        builder.build(StatementKind::Return { value: read_result });

        end_block_ref
            .get(function.block_arena_mut())
            .set_statements(builder.finish().into_iter());

        end_block_ref
    };

    // cyclic so need both created here
    let check_block_ref = function.new_block();
    let shift_block_ref = function.new_block();

    // check block if local_count == 0
    {
        let mut check_builder = StatementBuilder::new(check_block_ref);
        let _0 = check_builder.build(StatementKind::Constant {
            typ: (Type::u64()),
            value: ConstantValue::UnsignedInteger(0),
        });

        let read_count = check_builder.build(StatementKind::ReadVariable {
            symbol: local_count_symbol.clone(),
        });

        let count_is_zero = check_builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::CompareEqual,
            lhs: read_count.clone(),
            rhs: _0.clone(),
        });

        check_builder.build(StatementKind::Branch {
            condition: count_is_zero,
            true_target: end_block_ref,
            false_target: shift_block_ref,
        });

        check_block_ref
            .get(function.block_arena_mut())
            .set_statements(check_builder.finish().into_iter());
    }

    // decrement count
    {
        let mut shift_builder = StatementBuilder::new(shift_block_ref);
        let read_count = shift_builder.build(StatementKind::ReadVariable {
            symbol: local_count_symbol.clone(),
        });

        let _1 = shift_builder.build(StatementKind::Constant {
            typ: (Type::u64()),
            value: ConstantValue::UnsignedInteger(1),
        });

        let decrement = shift_builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::Sub,
            lhs: read_count.clone(),
            rhs: _1.clone(),
        });

        shift_builder.build(StatementKind::WriteVariable {
            symbol: local_count_symbol.clone(),
            value: decrement.clone(),
        });

        // read result and bits variables
        let read_result = shift_builder.build(StatementKind::ReadVariable {
            symbol: result_symbol.clone(),
        });

        let read_bits = shift_builder.build(StatementKind::ReadVariable {
            symbol: bits_symbol.clone(),
        });

        // get the length of bits, then cast from u8 to bundle
        let len = shift_builder.build(StatementKind::SizeOf {
            value: read_bits.clone(),
        });

        let _8 = shift_builder.build(StatementKind::Constant {
            typ: (Type::u8()),
            value: ConstantValue::UnsignedInteger(8),
        });

        let cast_len = shift_builder.build(StatementKind::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ: (Type::u8()),
            value: len.clone(),
        });

        let bundle_len = shift_builder.build(StatementKind::Cast {
            kind: CastOperationKind::Convert,
            typ: (Type::Bits),
            value: cast_len.clone(),
        });

        // shift result
        let shift_result = shift_builder.build(StatementKind::ShiftOperation {
            kind: ShiftOperationKind::LogicalShiftLeft,
            value: read_result.clone(),
            amount: bundle_len.clone(),
        });

        // or result with bits
        let or_result = shift_builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::Or,
            lhs: shift_result.clone(),
            rhs: read_bits.clone(),
        });

        // write result
        shift_builder.build(StatementKind::WriteVariable {
            symbol: result_symbol.clone(),
            value: or_result.clone(),
        });

        // jump
        shift_builder.build(StatementKind::Jump {
            target: check_block_ref,
        });

        shift_block_ref
            .get(function.block_arena_mut())
            .set_statements(shift_builder.finish().into_iter());
    }

    let entry_block = {
        let entry_block_ref = function.new_block();
        let mut entry_builder = StatementBuilder::new(entry_block_ref);
        // copy count to count_local
        // jump to check block
        let read_count = entry_builder.build(StatementKind::ReadVariable {
            symbol: count_symbol.clone(),
        });

        entry_builder.build(StatementKind::WriteVariable {
            symbol: local_count_symbol.clone(),
            value: read_count.clone(),
        });

        let zero = entry_builder.build(StatementKind::Constant {
            typ: (Type::u128()),
            value: ConstantValue::UnsignedInteger(0),
        });

        let bits = entry_builder.build(StatementKind::ReadVariable {
            symbol: bits_symbol.clone(),
        });

        let bits_length = entry_builder.build(StatementKind::SizeOf {
            value: bits.clone(),
        });

        let read_count_cast = entry_builder.build(StatementKind::Cast {
            kind: CastOperationKind::Truncate,
            typ: (Type::u16()),
            value: read_count.clone(),
        });

        let bits_length_cast = entry_builder.build(StatementKind::Cast {
            kind: CastOperationKind::Truncate,
            typ: (Type::u16()),
            value: bits_length.clone(),
        });

        let length = entry_builder.build(StatementKind::BinaryOperation {
            kind: BinaryOperationKind::Multiply,
            lhs: read_count_cast.clone(),
            rhs: bits_length_cast.clone(),
        });

        let result_bits = entry_builder.build(StatementKind::CreateBits {
            value: zero.clone(),
            length: length.clone(),
        });

        // write result
        entry_builder.build(StatementKind::WriteVariable {
            symbol: result_symbol.clone(),
            value: result_bits.clone(),
        });

        entry_builder.build(StatementKind::Jump {
            target: check_block_ref,
        });

        entry_block_ref
            .get(function.block_arena())
            .set_statements(entry_builder.finish().into_iter());

        entry_block_ref
    };

    function.entry_block = entry_block;

    function
});
