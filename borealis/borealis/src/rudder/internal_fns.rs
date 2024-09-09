use {
    crate::rudder::{
        constant_value::ConstantValue,
        statement::{
            BinaryOperationKind, CastOperationKind, ShiftOperationKind, StatementBuilder,
            StatementKind,
        },
        Block, Function, FunctionInner, Symbol, SymbolKind, Type,
    },
    common::{intern::InternedString, shared::Shared, HashMap},
    once_cell::sync::Lazy,
    std::sync::Arc,
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
        kind: SymbolKind::Parameter,
        typ: Arc::new(Type::Bits),
    };
    let count_symbol = Symbol {
        name: "count".into(),
        kind: SymbolKind::Parameter,
        typ: Arc::new(Type::u64()),
    };

    let local_count_symbol = Symbol {
        name: "local_count".into(),
        kind: SymbolKind::LocalVariable,
        typ: Arc::new(Type::u64()),
    };
    let result_symbol = Symbol {
        name: "result".into(),
        kind: SymbolKind::LocalVariable,
        typ: Arc::new(Type::Bits),
    };

    let end_block = {
        let end_block = Block::new();
        let mut builder = StatementBuilder::new(end_block.weak());

        let read_result = builder.build(StatementKind::ReadVariable {
            symbol: result_symbol.clone(),
        });

        builder.build(StatementKind::Return { value: read_result });

        end_block.set_statements(builder.finish().into_iter());

        end_block
    };

    // cyclic so need both created here
    let check_block = Block::new();
    let shift_block = Block::new();

    // check block if local_count == 0
    {
        let mut check_builder = StatementBuilder::new(check_block.weak());
        let _0 = check_builder.build(StatementKind::Constant {
            typ: Arc::new(Type::u64()),
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
            true_target: end_block,
            false_target: shift_block.clone(),
        });

        check_block.set_statements(check_builder.finish().into_iter());
    }

    // decrement count
    {
        let mut shift_builder = StatementBuilder::new(shift_block.weak());
        let read_count = shift_builder.build(StatementKind::ReadVariable {
            symbol: local_count_symbol.clone(),
        });

        let _1 = shift_builder.build(StatementKind::Constant {
            typ: Arc::new(Type::u64()),
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
            typ: Arc::new(Type::u8()),
            value: ConstantValue::UnsignedInteger(8),
        });

        let cast_len = shift_builder.build(StatementKind::Cast {
            kind: CastOperationKind::ZeroExtend,
            typ: Arc::new(Type::u8()),
            value: len.clone(),
        });

        let bundle_len = shift_builder.build(StatementKind::Cast {
            kind: CastOperationKind::Convert,
            typ: Arc::new(Type::Bits),
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
            target: check_block.clone(),
        });

        shift_block.set_statements(shift_builder.finish().into_iter());
    }

    let entry_block = {
        let entry_block = Block::new();
        let mut entry_builder = StatementBuilder::new(entry_block.weak());
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
            typ: Arc::new(Type::u128()),
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
            typ: Arc::new(Type::u16()),
            value: read_count.clone(),
        });

        let bits_length_cast = entry_builder.build(StatementKind::Cast {
            kind: CastOperationKind::Truncate,
            typ: Arc::new(Type::u16()),
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
            target: check_block.clone(),
        });

        entry_block.set_statements(entry_builder.finish().into_iter());

        entry_block
    };

    Function {
        inner: Shared::new(FunctionInner {
            name: InternedString::from_static("replicate_bits_borealis_internal"),

            local_variables: {
                let mut locals = HashMap::default();
                locals.insert(result_symbol.name(), result_symbol);
                locals.insert(local_count_symbol.name(), local_count_symbol);
                locals
            },
            entry_block,
        }),
        return_type: Arc::new(Type::Bits),
        parameters: vec![bits_symbol, count_symbol.clone()],
    }
});
