use {
    crate::{
        boom::{
            Ast, Expression, FunctionDefinition, Literal, Operation, OperationKind, Statement,
            Type, Value,
        },
        passes::builtin_fns::HandlerFunction,
    },
    common::{identifiable::unique_id, intern::InternedString, HashMap},
    itertools::Itertools,
    once_cell::sync::Lazy,
    std::{cell::RefCell, rc::Rc},
};

pub static HANDLERS: Lazy<HashMap<InternedString, HandlerFunction>> = Lazy::new(|| {
    let mappings = [
        // this is a GenC builtin function so can be left as a noop here (technically not needed
        // but just being explicit)
        ("trap", noop as HandlerFunction),
        // requires bitvector conversion logic
        ("UInt", replace_with_copy),
        // we represent integers as u64s so these can be removed
        ("pcnt_i___pcnt_i64", replace_with_copy),
        ("pcnt_i64___pcnt_i", replace_with_copy),
        // replace with equality test
        ("eq_int", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Equal)
        }),
        ("lt_int", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::LessThan)
        }),
        ("gteq_int", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::GreaterThanOrEqual)
        }),
        ("shl_int", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::LeftShift)
        }),
        ("eq_vec", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Equal)
        }),
        ("not_vec", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Complement)
        }),
        ("not_bool", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Not)
        }),
        ("add_atom", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Add)
        }),
        ("sub_atom", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Subtract)
        }),
        ("xor_vec", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Xor)
        }),
        ("or_vec", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Or)
        }),
        ("and_vec", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::And)
        }),
        ("add_vec", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::Add)
        }),
        ("neq_vec", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::NotEqual)
        }),
        ("ROR", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::RotateRight)
        }),
        ("ASR", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::RightShift)
        }),
        ("LSR", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::RightShift)
        }),
        ("LSL", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::LeftShift)
        }),
        // probably need to take another look at this
        ("SInt", replace_with_copy),
        ("bitvector_length", bv_length_handler),
        ("bitvector_access_B", bv_access_handler),
        ("bitvector_access_A", bv_access_handler),
        ("raw_GetSlice_int", noop),
        ("ZeroExtend__0", zero_extend_handler),
        ("ZeroExtend__1", zero_extend_handler),
        ("SignExtend__0", sign_extend_handler),
        ("slice", noop),
        ("Zeros", noop),
        ("undefined_bitvector", noop),
        ("bitvector_concat", noop),
        ("aget_PC", |ast, f, s| rename(ast, f, s, "read_pc".into())),
        ("sail_assert", assert_handler),
        ("undefined_bool", delete),
        ("undefined_LogicalOp", delete),
        ("replicate_bits", replicate_bits_handler),
        ("undefined_int", noop),
        ("undefined_real", noop),
        ("undefined_string", noop),
        ("undefined_unit", noop),
        ("ediv_int", noop),
        ("lteq_int", noop),
        ("eq_bool", noop),
        ("negate_atom", noop),
        ("mult_atom", noop),
        ("DecStr", noop),
        ("asl_prerr_string", noop),
        ("Error_Implementation_Defined", noop),
        ("Error_ExceptionTaken", noop),
        ("append_str", noop),
        ("GetVerbosity", noop),
        ("WriteRAM_bool", noop),
        ("cons", noop),
        ("Error_See", noop),
        ("update_fbits", noop),
        ("vector_access_A_B32_", noop),
        ("SetSlice_bits", noop),
        ("Error_Undefined", noop),
        ("shl8", noop),
        ("eq_anything_EFPRounding_pcnt__", noop),
        ("eq_real", noop),
        ("pcnt_string___pcnt_real", noop),
        ("quotient_real", noop),
        ("sub_real", noop),
        ("abs_real", noop),
        ("vector_update_B_B64_", noop),
        ("eq_anything_ECountOp_pcnt__", noop),
        ("eq_anything_EArchVersion_pcnt__", noop),
        ("vector_access_B_B32_", noop),
        ("sub_vec", noop),
        ("real_power", noop),
        ("internal_pick_EFPConvOp_pcnt__", noop),
        ("undefined_vector_RTLBLine_", noop),
        ("read_register_B32_", noop),
        ("Sqrt", noop),
        ("eq_anything_EConstraint_pcnt__", noop),
        ("eq_anything_EDeviceType_pcnt__", noop),
        ("eq_anything_EMBReqDomain_pcnt__", noop),
        ("Real", noop),
        ("internal_pick_EMemBarrierOp_pcnt__", noop),
        ("internal_pick_EMoveWideOp_pcnt__", noop),
        ("print_bits", noop),
        ("sail_zeros", noop),
        ("undefined_vector_B16_", noop),
        ("vector_subrange_A", noop),
        ("internal_pick_EInstrSet_pcnt__", noop),
        ("load_raw", noop),
        ("sub_vec_int", noop),
        ("emod_int", noop),
        ("eq_anything_EGTEParamType_pcnt__", noop),
        ("eq_anything_EMemOp_pcnt__", noop),
        ("vector_update_B_B128_", noop),
        ("internal_pick_EFPType_pcnt__", noop),
        ("internal_pick_EGTEParamType_pcnt__", noop),
        ("RoundUp", noop),
        ("internal_pick_EMBReqDomain_pcnt__", noop),
        ("internal_pick_EMemAtomicOp_pcnt__", noop),
        ("min_int", noop),
        ("internal_pick_ESystemHintOp_pcnt__", noop),
        ("internal_pick_EDeviceType_pcnt__", noop),
        ("pow2_atom", noop),
        ("gt_real", noop),
        ("undefined_vector_o_", noop),
        ("internal_pick_EBranchType_pcnt__", noop),
        ("vector_access_A_B128_", noop),
        ("eq_bit", noop),
        ("shl1", noop),
        ("get_cycle_count", noop),
        ("zeros", noop),
        ("internal_pick_EFPMaxMinOp_pcnt__", noop),
        ("vector_access_A_RTLBLine_", noop),
        ("internal_pick_ECompareOp_pcnt__", noop),
        ("max_int", noop),
        ("vector_access_B_i_", noop),
        ("eq_anything_Esignal_pcnt__", noop),
        ("vector_update_B_B32_", noop),
        ("eq_anything_EInstrSet_pcnt__", noop),
        ("print", noop),
        ("ReadRAM", noop),
        ("gteq_real", noop),
        ("shr32", noop),
        ("vector_update_B_o_", noop),
        ("eq_anything_ESRType_pcnt__", noop),
        ("internal_pick_EMBReqTypes_pcnt__", noop),
        ("internal_pick_EImmediateOp_pcnt__", noop),
        ("undefined_vector_B64_", noop),
        ("undefined_vector_B128_", noop),
        ("undefined_vector_i_", noop),
        ("vector_update_B_B16_", noop),
        ("check_cycle_count", noop),
        ("shr_int", noop),
        ("eq_string", noop),
        ("shl32", noop),
        ("WakeupRequest", noop),
        ("vector_update_subrange", noop),
        ("internal_pick_Esignal_pcnt__", noop),
        ("abs_int_atom", noop),
        ("concat_str", noop),
        ("vector_update_B_i_", noop),
        ("internal_pick_EConstraint_pcnt__", noop),
        ("internal_pick_EMemType_pcnt__", noop),
        ("vector_update_B_RTLBLine_", noop),
        ("internal_pick_EFPUnaryOp_pcnt__", noop),
        ("Error_Unpredictable", noop),
        ("eq_anything_EBranchType_pcnt__", noop),
        ("Error_ReservedEncoding", noop),
        ("gt_int", noop),
        ("SleepRequest", noop),
        ("vector_access_B_o_", noop),
        ("add_real", noop),
        ("eq_anything_EFault_pcnt__", noop),
        ("eq_anything_EMemType_pcnt__", noop),
        ("prerr_bits", noop),
        ("internal_pick_ELogicalOp_pcnt__", noop),
        ("eq_anything_EException_pcnt__", noop),
        ("internal_pick_EException_pcnt__", noop),
        ("RoundDown", noop),
        ("prerr_int", noop),
        ("zero_extend", noop),
        ("HexStr", noop),
        ("internal_pick_ESRType_pcnt__", noop),
        ("lt_real", noop),
        ("pcnt_string___pcnt_i", noop),
        ("vector_access_A__ref_B32_", noop),
        ("eq_anything_EAccType_pcnt__", noop),
        ("eq_anything_r_", noop),
        ("internal_pick_EAccType_pcnt__", noop),
        ("putchar", noop),
        ("mult_real", noop),
        ("vector_access_B_B16_", noop),
        ("internal_pick_EFault_pcnt__", noop),
        ("Error_SError", noop),
        ("add_vec_int", noop),
        ("internal_pick_EMemOp_pcnt__", noop),
        ("vector_access_B_B64_", noop),
        ("vector_subrange_B", noop),
        ("SetSlice_int", noop),
        ("eq_anything_EMoveWideOp_pcnt__", noop),
        ("vector_access_A_B64_", noop),
        ("negate_real", noop),
        ("undefined_vector_B32_", noop),
        ("vector_access_B_RTLBLine_", noop),
        ("eq_anything_EFPType_pcnt__", noop),
        ("vector_access_A_o_", noop),
        ("internal_pick_EPSTATEField_pcnt__", noop),
        ("internal_pick_EPrefetchHint_pcnt__", noop),
        ("internal_pick_EVBitOp_pcnt__", noop),
        ("internal_pick_EFPRounding_pcnt__", noop),
    ]
    .into_iter()
    .map(|(s, f)| (InternedString::from_static(s), f));

    assert!(
        mappings.clone().map(|(n, _)| n).all_unique(),
        "non-unique function name found in mappings to handler!"
    );

    HashMap::from_iter(mappings)
});

pub fn noop(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    _statement: Rc<RefCell<Statement>>,
) {
}

pub fn delete(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    let (block, index) = function.entry_block.find_statement(statement).unwrap();
    let mut statements = block.statements();
    statements.remove(index);
    block.set_statements(statements);
}

/// Blindly replace function call with assignment
pub fn replace_with_copy(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    // get function arguments, and expression the function output is being assigned
    // to
    let (expression, arguments) = {
        let Statement::FunctionCall {
            expression,
            arguments,
            ..
        } = &*statement.borrow()
        else {
            panic!("these should all be function call statements")
        };

        (expression.clone(), arguments.clone())
    };

    // expression should be Some
    let Some(expression) = expression else {
        panic!("no lhs");
    };

    // // should *probably* be an identifier, but we could allow any expression kind
    // // here
    // let Expression::Identifier(_) = expression else {
    //     panic!("non ident expression");
    // };

    // can only do this with a single argument
    assert_eq!(arguments.len(), 1);

    // turn it into a copy
    *statement.borrow_mut() = Statement::Copy {
        expression: expression.clone(),
        value: arguments[0].clone(),
    };
}

///
pub fn replace_with_op(
    _: Rc<RefCell<Ast>>,
    _: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
    operator: OperationKind,
) {
    // assert there are two arguments, lhs and rhs
    // replace function call with copy where the value is an equals operation

    let (expression, args) = {
        let Statement::FunctionCall {
            expression: Some(expression),
            arguments,
            ..
        } = &*statement.borrow()
        else {
            panic!();
        };

        (expression.clone(), arguments.clone())
    };

    let operation = match operator {
        OperationKind::Not => Operation::Not(args[0].clone()),
        OperationKind::Complement => Operation::Complement(args[0].clone()),
        OperationKind::Equal => Operation::Equal(args[0].clone(), args[1].clone()),
        OperationKind::NotEqual => Operation::NotEqual(args[0].clone(), args[1].clone()),
        OperationKind::LessThan => Operation::LessThan(args[0].clone(), args[1].clone()),
        OperationKind::GreaterThan => Operation::GreaterThan(args[0].clone(), args[1].clone()),
        OperationKind::LessThanOrEqual => {
            Operation::LessThanOrEqual(args[0].clone(), args[1].clone())
        }
        OperationKind::GreaterThanOrEqual => {
            Operation::GreaterThanOrEqual(args[0].clone(), args[1].clone())
        }
        OperationKind::Subtract => Operation::Subtract(args[0].clone(), args[1].clone()),
        OperationKind::Add => Operation::Add(args[0].clone(), args[1].clone()),
        OperationKind::Or => Operation::Or(args[0].clone(), args[1].clone()),
        OperationKind::And => Operation::And(args[0].clone(), args[1].clone()),
        OperationKind::LeftShift => Operation::LeftShift(args[0].clone(), args[1].clone()),
        OperationKind::RightShift => Operation::RightShift(args[0].clone(), args[1].clone()),
        OperationKind::Xor => Operation::Xor(args[0].clone(), args[1].clone()),
        OperationKind::RotateLeft => Operation::RotateLeft(args[0].clone(), args[1].clone()),
        OperationKind::RotateRight => Operation::RotateRight(args[0].clone(), args[1].clone()),
        OperationKind::Cast => panic!(),
    };

    *statement.borrow_mut() = Statement::Copy {
        expression,
        value: Rc::new(RefCell::new(Value::Operation(operation))),
    };
}

///
pub fn bv_length_handler(
    _: Rc<RefCell<Ast>>,
    fn_def: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    // get ident of argument from statement, use that to lookup the type, which
    // should have a size

    let Statement::FunctionCall {
        expression,
        arguments,
        ..
    } = statement.borrow().clone()
    else {
        panic!();
    };

    assert_eq!(arguments.len(), 1);

    let Value::Identifier(bv_ident) = &*arguments[0].borrow() else {
        panic!();
    };

    let len = match fn_def.get_ident_type(*bv_ident).unwrap() {
        Type::FixedInt(len) | Type::FixedBits(len, _) => len,
        // TODO: this is a lie and wrong
        Type::LargeBits(_) | Type::LargeInt => 64,
        _ => panic!("not a bv"),
    };

    *statement.borrow_mut() = Statement::Copy {
        expression: expression.as_ref().unwrap().clone(),
        value: Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
            Literal::Int(len.into()),
        ))))),
    };

    // replace statement with a copy of that size as a literal to the original
    // destination
}

pub fn bv_access_handler(
    _: Rc<RefCell<Ast>>,
    _: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    // access a bit of a bitvector by generating the appropriate shifting logic
    // dest = bitvector_access_B(input, index);
    //
    // dest = (input >> index) & 1

    let Statement::FunctionCall {
        expression,
        arguments,
        ..
    } = statement.borrow().clone()
    else {
        panic!();
    };

    *statement.borrow_mut() = Statement::Copy {
        expression: expression.unwrap(),
        value: Rc::new(RefCell::new(Value::Operation(Operation::And(
            Rc::new(RefCell::new(Value::Operation(Operation::RightShift(
                arguments[0].clone(),
                arguments[1].clone(),
            )))),
            Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
                Literal::Int(1.into()),
            ))))),
        )))),
    }
}

pub fn zero_extend_handler(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    let Statement::FunctionCall {
        expression,
        arguments,
        ..
    } = statement.borrow().clone()
    else {
        panic!();
    };

    // x1 = ZeroExtend__0(x, n)
    //
    //

    let Value::Identifier(x_ident) = &*arguments[0].borrow() else {
        panic!();
    };

    let x_len = match function.get_ident_type(*x_ident).unwrap() {
        Type::FixedBits(len, _) | Type::FixedSignedBits(len) => len,
        _ => return,
    };

    // (x << (64 - x_len)) >> (64 - x_len)

    *statement.borrow_mut() = Statement::Copy {
        expression: expression.unwrap(),
        value: Rc::new(RefCell::new(Value::Operation(Operation::RightShift(
            Rc::new(RefCell::new(Value::Operation(Operation::LeftShift(
                Rc::new(RefCell::new(Value::Operation(Operation::Cast(
                    arguments[0].clone(),
                    Rc::new(RefCell::new(Type::FixedBits(64, false))),
                )))),
                Rc::new(RefCell::new(Value::Operation(Operation::Subtract(
                    arguments[1].clone(),
                    Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
                        Literal::Int(x_len.into()),
                    ))))),
                )))),
            )))),
            Rc::new(RefCell::new(Value::Operation(Operation::Subtract(
                arguments[1].clone(),
                Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
                    Literal::Int(x_len.into()),
                ))))),
            )))),
        )))),
    };
}

pub fn sign_extend_handler(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    let Statement::FunctionCall {
        expression,
        arguments,
        ..
    } = statement.borrow().clone()
    else {
        panic!();
    };

    // x1 = ZeroExtend__0(x, n)
    //
    //

    let Value::Identifier(x_ident) = &*arguments[0].borrow() else {
        panic!();
    };

    let Type::FixedBits(x_len, _) = function.get_ident_type(*x_ident).unwrap() else {
        panic!();
    };

    // ((sint64)x << (64 - x_len)) >> (64 - x_len)

    *statement.borrow_mut() = Statement::Copy {
        expression: expression.unwrap(),
        value: Rc::new(RefCell::new(Value::Operation(Operation::RightShift(
            Rc::new(RefCell::new(Value::Operation(Operation::LeftShift(
                Rc::new(RefCell::new(Value::Operation(Operation::Cast(
                    arguments[0].clone(),
                    Rc::new(RefCell::new(Type::FixedSignedBits(64))),
                )))),
                Rc::new(RefCell::new(Value::Operation(Operation::Subtract(
                    arguments[1].clone(),
                    Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
                        Literal::Int(x_len.into()),
                    ))))),
                )))),
            )))),
            Rc::new(RefCell::new(Value::Operation(Operation::Subtract(
                arguments[1].clone(),
                Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
                    Literal::Int(x_len.into()),
                ))))),
            )))),
        )))),
    };
}

fn rename(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
    new_name: InternedString,
) {
    let Statement::FunctionCall { name, .. } = &mut *statement.borrow_mut() else {
        panic!();
    };

    *name = new_name;
}

pub fn assert_handler(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    let Statement::FunctionCall {
        expression,
        arguments,
        ..
    } = &mut *statement.borrow_mut()
    else {
        panic!();
    };

    *expression = None;
    arguments.pop();
}

pub fn replicate_bits_handler(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    let (expression, name, length, count) = {
        let Statement::FunctionCall {
            expression,
            arguments,
            ..
        } = &*statement.borrow()
        else {
            panic!();
        };

        let Value::Identifier(ident) = &*arguments[1].borrow() else {
            panic!();
        };

        let Some(value) = function.entry_block.get_assignment(*ident) else {
            return;
        };

        let Value::Literal(literal) = &*value.borrow() else {
            panic!();
        };

        let Literal::Int(count) = &*literal.borrow() else {
            panic!();
        };

        let Value::Identifier(ident) = &*arguments[0].borrow() else {
            panic!();
        };

        let Some(Type::FixedBits(length, _)) = function.get_ident_type(*ident) else {
            panic!();
        };

        (
            expression.clone().unwrap(),
            *ident,
            length,
            isize::try_from(count).unwrap(),
        )
    };

    let (block, idx) = function
        .entry_block
        .find_statement(statement.clone())
        .unwrap();

    let mut statements = block.statements();

    statements.remove(idx);

    let mut rep_statements = vec![];

    let id = unique_id();

    let mut prev_ident = format!("rep_{id}_0").into();
    rep_statements.push(Rc::new(RefCell::new(Statement::TypeDeclaration {
        name: prev_ident,
        typ: Rc::new(RefCell::new(Type::FixedBits(64, false))),
    })));
    rep_statements.push(Rc::new(RefCell::new(Statement::Copy {
        expression: Expression::Identifier(prev_ident),
        value: Rc::new(RefCell::new(Value::Identifier(name))),
    })));

    for i in 1..count {
        let this_ident = format!("rep_{id}_{i}").into();
        rep_statements.push(Rc::new(RefCell::new(Statement::TypeDeclaration {
            name: this_ident,
            typ: Rc::new(RefCell::new(Type::FixedBits(64, false))),
        })));
        rep_statements.push(Rc::new(RefCell::new(Statement::Copy {
            expression: Expression::Identifier(this_ident),
            value: Rc::new(RefCell::new(Value::Operation(Operation::Or(
                Rc::new(RefCell::new(Value::Operation(Operation::LeftShift(
                    Rc::new(RefCell::new(Value::Identifier(name))),
                    Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
                        Literal::Int((length * i).into()),
                    ))))),
                )))),
                Rc::new(RefCell::new(Value::Identifier(prev_ident))),
            )))),
        })));
        prev_ident = this_ident;
    }

    rep_statements.push(Rc::new(RefCell::new(Statement::Copy {
        expression,
        value: Rc::new(RefCell::new(Value::Identifier(prev_ident))),
    })));

    statements.splice(idx..idx, rep_statements);
    block.set_statements(statements);
}
