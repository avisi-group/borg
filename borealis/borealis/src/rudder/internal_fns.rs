use {
    crate::rudder::{
        BinaryOperationKind, Block, CastOperationKind, ConstantValue, Function, FunctionInner,
        ShiftOperationKind, Statement, StatementInner, StatementKind, Symbol, SymbolKind, Type,
    },
    common::{intern::InternedString, HashMap},
    once_cell::sync::Lazy,
    std::{cell::RefCell, rc::Rc},
};

// terrible
unsafe impl Send for Function {}
unsafe impl Sync for Function {}

pub const REPLICATE_BITS_BOREALIS_INTERNAL_NAME: Lazy<InternedString> =
    Lazy::new(|| InternedString::from_static("replicate_bits_borealis_internal"));

pub static REPLICATE_BITS_BOREALIS_INTERNAL: Lazy<Function> = Lazy::new(|| {
    // // bits << (bits.len() * 0) | bits << (bits.len() * 1) | bits << (bits.len()
    // * 2) ...

    // todo!();

    // for i in 0..count {
    //     acc <<= bits.len();
    //     acc |= bits;
    // }

    let bits_symbol = Symbol {
        name: "bits".into(),
        kind: SymbolKind::Parameter,
        typ: Rc::new(Type::Bundled {
            value: Rc::new(Type::u64()),
            len: Rc::new(Type::u8()),
        }),
    };
    let count_symbol = Symbol {
        name: "count".into(),
        kind: SymbolKind::Parameter,
        typ: Rc::new(Type::u64()),
    };

    let local_count_symbol = Symbol {
        name: "local_count".into(),
        kind: SymbolKind::LocalVariable,
        typ: Rc::new(Type::u64()),
    };
    let result_symbol = Symbol {
        name: "result".into(),
        kind: SymbolKind::LocalVariable,
        typ: Rc::new(Type::Bundled {
            value: Rc::new(Type::u64()),
            len: Rc::new(Type::u8()),
        }),
    };

    let end_block = {
        let end_block = Block::new();

        let read_result = Statement {
            inner: Rc::new(RefCell::new(StatementInner {
                name: "s0".into(),
                kind: StatementKind::ReadVariable {
                    symbol: result_symbol.clone(),
                },
                parent: end_block.weak(),
            })),
        };

        end_block.set_statements(
            [
                read_result.clone(),
                Statement {
                    inner: Rc::new(RefCell::new(StatementInner {
                        name: "s1".into(),
                        kind: StatementKind::Return {
                            value: Some(read_result),
                        },
                        parent: end_block.weak(),
                    })),
                },
            ]
            .into_iter(),
        );

        end_block
    };

    // cyclic so need both created here
    let check_block = Block::new();
    let shift_block = Block::new();

    // check block if local_count == 0

    let _0 = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s2".into(),
            kind: StatementKind::Constant {
                typ: Rc::new(Type::u64()),
                value: ConstantValue::UnsignedInteger(0),
            },
            parent: check_block.weak(),
        })),
    };

    let read_count = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s3".into(),
            kind: StatementKind::ReadVariable {
                symbol: local_count_symbol.clone(),
            },
            parent: check_block.weak(),
        })),
    };
    let count_is_0 = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s4".into(),
            kind: StatementKind::BinaryOperation {
                kind: BinaryOperationKind::CompareEqual,
                lhs: read_count.clone(),
                rhs: _0.clone(),
            },
            parent: check_block.weak(),
        })),
    };

    check_block.set_statements(
        [
            _0,
            read_count,
            count_is_0.clone(),
            Statement {
                inner: Rc::new(RefCell::new(StatementInner {
                    name: "s5".into(),
                    kind: StatementKind::Branch {
                        condition: count_is_0,
                        true_target: end_block.clone(),
                        false_target: shift_block.clone(),
                    },
                    parent: check_block.weak(),
                })),
            },
        ]
        .into_iter(),
    );

    // decrement count
    let read_count = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s6".into(),
            kind: StatementKind::ReadVariable {
                symbol: local_count_symbol.clone(),
            },
            parent: shift_block.weak(),
        })),
    };
    let _1 = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s7".into(),
            kind: StatementKind::Constant {
                typ: Rc::new(Type::u64()),
                value: ConstantValue::UnsignedInteger(1),
            },
            parent: shift_block.weak(),
        })),
    };
    let decrement = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s8".into(),
            kind: StatementKind::BinaryOperation {
                kind: BinaryOperationKind::Sub,
                lhs: read_count.clone(),
                rhs: _1.clone(),
            },
            parent: shift_block.weak(),
        })),
    };
    let write_count = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s9".into(),
            kind: StatementKind::WriteVariable {
                symbol: local_count_symbol.clone(),
                value: decrement.clone(),
            },
            parent: shift_block.weak(),
        })),
    };

    // read result and bits variables
    let read_result = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s10".into(),
            kind: StatementKind::ReadVariable {
                symbol: result_symbol.clone(),
            },
            parent: shift_block.weak(),
        })),
    };
    let read_bits = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s11".into(),
            kind: StatementKind::ReadVariable {
                symbol: bits_symbol.clone(),
            },
            parent: shift_block.weak(),
        })),
    };

    // get the length of bits, then cast from u8 to bundle
    let len = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s12".into(),
            kind: StatementKind::UnbundleLength {
                bundle: read_bits.clone(),
            },
            parent: shift_block.weak(),
        })),
    };
    let _8 = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s12.5".into(),
            kind: StatementKind::Constant {
                typ: Rc::new(Type::u8()),
                value: ConstantValue::UnsignedInteger(8),
            },
            parent: shift_block.weak(),
        })),
    };
    let cast_len = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s13".into(),
            kind: StatementKind::Cast {
                kind: CastOperationKind::ZeroExtend,
                typ: Rc::new(Type::u64()),
                value: len.clone(),
            },
            parent: shift_block.weak(),
        })),
    };
    let bundle_len = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s13".into(),
            kind: StatementKind::Bundle {
                value: cast_len.clone(),
                length: _8.clone(),
            },
            parent: shift_block.weak(),
        })),
    };

    // shift result
    let shift_result = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s14".into(),
            kind: StatementKind::ShiftOperation {
                kind: ShiftOperationKind::LogicalShiftLeft,
                value: read_result.clone(),
                amount: bundle_len.clone(),
            },
            parent: shift_block.weak(),
        })),
    };

    // or result with bits
    let or_result = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s15".into(),
            kind: StatementKind::BinaryOperation {
                kind: BinaryOperationKind::Or,
                lhs: shift_result.clone(),
                rhs: read_bits.clone(),
            },
            parent: shift_block.weak(),
        })),
    };

    let write_result = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s16".into(),
            kind: StatementKind::WriteVariable {
                symbol: result_symbol.clone(),
                value: or_result.clone(),
            },
            parent: shift_block.weak(),
        })),
    };

    let jump = Statement {
        inner: Rc::new(RefCell::new(StatementInner {
            name: "s17".into(),
            kind: StatementKind::Jump {
                target: check_block.clone(),
            },
            parent: shift_block.weak(),
        })),
    };

    shift_block.set_statements(
        [
            read_count,
            _1,
            decrement,
            write_count,
            read_result,
            read_bits,
            _8,
            len,
            cast_len,
            bundle_len,
            shift_result,
            or_result,
            write_result,
            jump,
        ]
        .into_iter(),
    );

    let entry_block = {
        let entry_block = Block::new();
        // copy count to count_local
        // jump to check block
        let read_count = Statement {
            inner: Rc::new(RefCell::new(StatementInner {
                name: "s3".into(),
                kind: StatementKind::ReadVariable {
                    symbol: count_symbol.clone(),
                },
                parent: entry_block.weak(),
            })),
        };
        let write_local_count = Statement {
            inner: Rc::new(RefCell::new(StatementInner {
                name: "s3".into(),
                kind: StatementKind::WriteVariable {
                    symbol: local_count_symbol.clone(),
                    value: read_count.clone(),
                },
                parent: entry_block.weak(),
            })),
        };

        entry_block.set_statements(
            [
                read_count,
                write_local_count,
                Statement {
                    inner: Rc::new(RefCell::new(StatementInner {
                        name: "s3".into(),
                        kind: StatementKind::Jump {
                            target: check_block.clone(),
                        },
                        parent: entry_block.weak(),
                    })),
                },
            ]
            .into_iter(),
        );

        entry_block
    };

    Function {
        inner: Rc::new(RefCell::new(FunctionInner {
            name: *REPLICATE_BITS_BOREALIS_INTERNAL_NAME,
            return_type: Rc::new(Type::Bundled {
                value: Rc::new(Type::u64()),
                len: Rc::new(Type::u8()),
            }),
            parameters: vec![bits_symbol, count_symbol.clone()],
            local_variables: {
                let mut locals = HashMap::default();
                locals.insert(result_symbol.name(), result_symbol);
                locals.insert(local_count_symbol.name(), local_count_symbol);
                locals
            },
            entry_block,
        })),
    }
});
