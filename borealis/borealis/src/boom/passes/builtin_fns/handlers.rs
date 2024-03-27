use {
    crate::boom::{
        control_flow::{ControlFlowBlock, Terminator},
        passes::builtin_fns::HandlerFunction,
        Ast, FunctionDefinition, Literal, Operation, OperationKind, Size, Statement, Type, Value,
    },
    common::{intern::InternedString, HashMap},
    itertools::Itertools,
    once_cell::sync::Lazy,
    std::{cell::RefCell, rc::Rc},
};

pub static HANDLERS: Lazy<HashMap<InternedString, HandlerFunction>> = Lazy::new(|| {
    let mappings = [
        ("trap", noop as HandlerFunction),
        // // requires bitvector conversion logic
        // ("UInt", replace_with_copy),
        // // we represent integers as u64s so these can be removed
        // ("u_pcnt_i___pcnt_i64", replace_with_copy),
        // ("u_pcnt_i64___pcnt_i", replace_with_copy),
        // // replace with equality test
        // ("eq_bool", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("eq_string", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("eq_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("neq_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::NotEqual)
        // }),
        ("lt_int", |ast, f, s| {
            replace_with_op(ast, f, s, OperationKind::LessThan)
        }),
        // ("gt_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::GreaterThan)
        // }),
        // ("lteq_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::LessThanOrEqual)
        // }),
        // ("gteq_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::GreaterThanOrEqual)
        // }),
        // ("u_shl_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::LeftShift)
        // }),
        // ("u_shl8", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::LeftShift)
        // }),
        // ("eq_vec", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("not_vec", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Complement)
        // }),
        // ("not_bool", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Not)
        // }),
        // ("add_atom", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Add)
        // }),
        // ("sub_atom", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Subtract)
        // }),
        // ("mult_atom", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Multiply)
        // }),
        // ("xor_vec", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Xor)
        // }),
        // ("or_vec", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Or)
        // }),
        // ("and_vec", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::And)
        // }),
        // ("add_vec", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Add)
        // }),
        // ("add_vec_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Add)
        // }),
        // ("neq_vec", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::NotEqual)
        // }),
        // ("ROR", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::RotateRight)
        // }),
        // ("ASR", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::RightShift)
        // }),
        // ("LSR", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::RightShift)
        // }),
        // ("LSL", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::LeftShift)
        // }),
        // ("ediv_int", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Divide)
        // }),
        // ("SInt", replace_with_copy), // probably need to take another look at this
        // ("bitvector_length", bv_length_handler),
        // ("bitvector_access_B", bv_access_handler),
        // ("bitvector_access_A", bv_access_handler),
        // ("raw_GetSlice_int", noop),
        // ("ZeroExtend__0", zero_extend_handler),
        // ("ZeroExtend__1", zero_extend_handler),
        // ("SignExtend__0", sign_extend_handler),
        // ("eq_anything_EArchVersion_pcnt__", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("eq_anything_EMoveWideOp_pcnt__", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("eq_anything_EBranchType_pcnt__", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("eq_anything_EMemOp_pcnt__", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("eq_anything_EConstraint_pcnt__", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::Equal)
        // }),
        // ("neq_anything_EMemOp_pcnt__", |ast, f, s| {
        //     replace_with_op(ast, f, s, OperationKind::NotEqual)
        // }),
        // ("slice", noop),
        // ("Zeros", noop),
        // ("undefined_bitvector", noop),
        // ("bitvector_concat", noop),
        // ("aget_PC", |ast, f, s| rename(ast, f, s, "read_pc".into())),
        // // these two handlers originally replaced the call with an "inline" if statement, rather
        // // than splitting the control flow blocks, etc
        // ("sail_assert", assert_handler),
        // ("min_int", noop),
        // //
        // // delete undefined initializers
        // ("undefined_bool", delete),
        // ("undefined_LogicalOp", delete),
        // ("undefined_int", delete),
        // ("undefined_MoveWideOp", delete),
        // ("undefined_MemOp", delete),
        // ("undefined_Constraint", delete),
        // ("undefined_SystemHintOp", delete),
        // ("undefined_BranchType", delete),
        // ("replicate_bits", replicate_bits_handler),
        // ("SetSlice_int", noop),
        // ("u__SetSlice_bits", set_slice_handler),
        // ("Extend__0", extend_handler),
        // ("update_fbits", update_fbits_handler),
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
) -> bool {
    false
}

pub fn delete(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
    let (block, index) = function.entry_block.find_statement(statement).unwrap();
    let mut statements = block.statements();
    statements.remove(index);
    block.set_statements(statements);
    false
}

/// Blindly replace function call with assignment
pub fn replace_with_copy(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
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

    true
}

///
pub fn replace_with_op(
    _: Rc<RefCell<Ast>>,
    _: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
    operator: OperationKind,
) -> bool {
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
        OperationKind::Multiply => Operation::Multiply(args[0].clone(), args[1].clone()),
        OperationKind::Divide => Operation::Divide(args[0].clone(), args[1].clone()),
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
        value: operation.into(),
    };

    true
}

///
pub fn bv_length_handler(
    _: Rc<RefCell<Ast>>,
    fn_def: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
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

    let size = match fn_def.get_ident_type(*bv_ident).unwrap() {
        Type::Int {
            size: Size::Static(size),
            ..
        } => Literal::Int(size.into()).into(),
        Type::Int {
            size: Size::Runtime(value),
            ..
        } => value,
        Type::Int {
            size: Size::Unknown,
            ..
        } => Literal::Int(64.into()).into(),
        _ => panic!("not a bv"),
    };

    // replace statement with a copy of that size as a literal to the original
    // destination
    *statement.borrow_mut() = Statement::Copy {
        expression: expression.as_ref().unwrap().clone(),
        value: size,
    };

    true
}

pub fn bv_access_handler(
    _: Rc<RefCell<Ast>>,
    _: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
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
        value: Operation::And(
            Operation::RightShift(arguments[0].clone(), arguments[1].clone()).into(),
            Literal::Int(1.into()).into(),
        )
        .into(),
    };

    true
}

pub fn zero_extend_handler(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
    let Statement::FunctionCall {
        expression,
        arguments,
        name,
    } = statement.borrow().clone()
    else {
        panic!();
    };

    // zero-extend value to length bits
    let (value, length) = if name.as_ref() == "ZeroExtend__0" {
        (arguments[0].clone(), arguments[1].clone())
    } else if name.as_ref() == "ZeroExtend__1" {
        (arguments[1].clone(), arguments[0].clone())
    } else {
        panic!();
    };

    let Value::Identifier(value_ident) = &*value.borrow() else {
        return false;
    };

    let value_len = match function.get_ident_type(*value_ident).unwrap() {
        Type::Int {
            size: Size::Static(len),
            ..
        } => Literal::Int(len.into()).into(),
        Type::Int {
            size: Size::Runtime(value),
            ..
        } => value,
        // TODO: write comment proving why this is false not true
        _ => return false,
    };

    let size = match &*length.borrow() {
        Value::Literal(lit) => {
            if let Literal::Int(size) = &*lit.borrow() {
                Size::Static(size.try_into().unwrap())
            } else {
                Size::Runtime(length.clone())
            }
        }
        _ => Size::Runtime(length.clone()),
    };

    *statement.borrow_mut() = Statement::Copy {
        expression: expression.unwrap(),
        value: Operation::RightShift(
            Operation::LeftShift(
                Operation::Cast(
                    arguments[0].clone(),
                    Rc::new(RefCell::new(Type::Int {
                        signed: false,
                        size,
                    })),
                )
                .into(),
                Operation::Subtract(length.clone(), value_len.clone()).into(),
            )
            .into(),
            Operation::Subtract(length, value_len).into(),
        )
        .into(),
    };

    true
}

pub fn sign_extend_handler(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
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

    let len_value: Rc<RefCell<Value>> =
        if let Type::Int { size, .. } = function.get_ident_type(*x_ident).unwrap() {
            match size {
                Size::Static(size) => Literal::Int(size.into()).into(),
                Size::Runtime(value) => value,
                Size::Unknown => panic!("unknown size {x_ident:?} in {}", function.signature.name),
            }
        } else {
            panic!(
                "could not get type of {x_ident:?} in {}",
                function.signature.name
            );

            // function.entry_block.get_assignment(ident)
            // Rc::new(RefCell::new(Value::Literal(Rc::new(RefCell::new(
            //     Literal::Int(x_len.into()),
            // )))))
        };

    // ((sint64)x << (64 - x_len)) >> (64 - x_len)

    *statement.borrow_mut() = Statement::Copy {
        expression: expression.unwrap(),
        value: Operation::RightShift(
            Operation::Cast(
                Operation::LeftShift(
                    Operation::Cast(
                        arguments[0].clone(),
                        Rc::new(RefCell::new(Type::Int {
                            signed: true,
                            size: Size::Static(64),
                        })),
                    )
                    .into(),
                    Operation::Subtract(arguments[1].clone(), len_value.clone()).into(),
                )
                .into(),
                Rc::new(RefCell::new(Type::Int {
                    signed: true,
                    size: Size::Static(64),
                })),
            )
            .into(),
            Operation::Subtract(arguments[1].clone(), len_value).into(),
        )
        .into(),
    };

    true
}

fn rename(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
    new_name: InternedString,
) -> bool {
    let Statement::FunctionCall { name, .. } = &mut *statement.borrow_mut() else {
        panic!();
    };

    *name = new_name;

    true
}

/// Inserts size field
pub fn replicate_bits_handler(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
    let Statement::FunctionCall { arguments, .. } = &mut *statement.borrow_mut() else {
        panic!();
    };

    let size = {
        let Value::Identifier(ident) = &*arguments[0].borrow() else {
            panic!("{:?}", arguments[0]);
        };

        let Some(Type::Int { size, .. }) = function.get_ident_type(*ident) else {
            panic!();
        };

        size
    };

    if arguments.len() == 2 {
        arguments.insert(1, size.try_into().unwrap());
        true
    } else {
        false
    }
}

pub fn set_slice_handler(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
    // result = 0;
    // // RefCell { value: LargeInt }
    // uint64 gs_12779;
    // // Identifier("gs_12779") RefCell { value: Literal(RefCell { value: Int(16)
    // }) } gs_12779 = 16;
    // // RefCell { value: LargeInt }
    // uint64 gs_155056;
    // // Identifier("gs_155056") RefCell { value: Identifier("datasize") }
    // gs_155056 = datasize;
    // // RefCell { value: FixedBits(16, false) }
    // uint16 gs_155057;
    // // Identifier("gs_155057") RefCell { value: Identifier("imm") }
    // gs_155057 = imm;
    // result = SetSlice_bits(gs_155056, gs_12779, result, pos, gs_155057);

    // void set_slice(lbits *rop,
    //     const sail_int len_mpz,
    //     const sail_int slen_mpz,
    //     const lbits op,
    //     const sail_int start_mpz,
    //     const lbits slice)

    // result = (result & (~(((1 << slice_len) - 1) << pos))) | (slice << pos)
    let (expression, slice_len, input, pos, slice) = {
        let Statement::FunctionCall {
            expression,

            arguments,
            ..
        } = &*statement.borrow()
        else {
            panic!();
        };

        (
            expression.clone().unwrap(),
            arguments[1].clone(),
            arguments[2].clone(),
            arguments[3].clone(),
            arguments[4].clone(),
        )
    };

    *statement.borrow_mut() = Statement::Copy {
        expression,
        value: Operation::Or(
            Operation::And(
                input,
                Operation::Complement(
                    Operation::LeftShift(
                        Operation::Subtract(
                            Operation::LeftShift(Literal::Int(1.into()).into(), slice_len).into(),
                            Literal::Int(1.into()).into(),
                        )
                        .into(),
                        pos.clone(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
            Operation::LeftShift(slice, pos).into(),
        )
        .into(),
    };

    true
}

pub fn extend_handler(
    _: Rc<RefCell<Ast>>,
    fn_def: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
    // val Extend__0 : (%bv, %i, %bool) -> %bv
    // fn Extend__0(x, N, unsigned) {
    //     if (unsigned) {
    //       return = ZeroExtend__0(x, N)
    //     } else {
    //       return = SignExtend__0(x, N)
    //     }
    //     label "end_function_4997"
    //     end(return)
    //     label "end_block_exception_4998"
    //     undefined
    //   }

    // set terminator to be conditional on `unsigned`

    // create two new blocks with calls to extend fns

    // set terminators to the second half of the original split block

    let (destination, x, n, unsigned) = {
        let Statement::FunctionCall {
            expression,
            arguments,
            ..
        } = &*statement.borrow()
        else {
            panic!();
        };

        (
            expression.clone(),
            arguments[0].clone(),
            arguments[1].clone(),
            arguments[2].clone(),
        )
    };

    let (block, pos) = fn_def.entry_block.find_statement(statement).unwrap();

    // split block statements at pos
    let mut statements = block.statements();
    statements.remove(pos);
    let (pre_statements, post_statements) = statements.split_at(pos);

    let final_block = ControlFlowBlock::new();
    final_block.set_statements(post_statements.to_owned());
    final_block.set_terminator(block.terminator());

    let target = ControlFlowBlock::new();
    target.set_statements(vec![Rc::new(RefCell::new(Statement::FunctionCall {
        expression: destination.clone(),
        name: "ZeroExtend__0".into(),
        arguments: vec![x.clone(), n.clone()],
    }))]);
    target.set_terminator(Terminator::Unconditional {
        target: final_block.clone(),
    });

    let fallthrough = ControlFlowBlock::new();
    fallthrough.set_statements(vec![Rc::new(RefCell::new(Statement::FunctionCall {
        expression: destination.clone(),
        name: "SignExtend__0".into(),
        arguments: vec![x, n],
    }))]);
    fallthrough.set_terminator(Terminator::Unconditional {
        target: final_block,
    });

    block.set_statements(pre_statements.to_owned());
    block.set_terminator(Terminator::Conditional {
        condition: unsigned.borrow().clone(),
        target,
        fallthrough,
    });

    true
}

pub fn update_fbits_handler(
    _: Rc<RefCell<Ast>>,
    fn_def: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
    let (destination, op, n, bit) = {
        let Statement::FunctionCall {
            expression,
            arguments,
            ..
        } = &*statement.borrow()
        else {
            panic!();
        };

        (
            expression.clone(),
            arguments[0].clone(),
            arguments[1].clone(),
            arguments[2].clone(),
        )
    };

    // assert op variable assigned to 0
    {
        let Value::Identifier(ident) = &*op.borrow() else {
            panic!();
        };

        let assignment = fn_def.entry_block.get_assignment(*ident).unwrap();

        let Value::Literal(literal) = &*assignment.borrow() else {
            panic!();
        };

        let Literal::Int(n) = &*literal.borrow() else {
            panic!();
        };

        assert_eq!(u64::try_from(n).unwrap(), 0);
    }

    // assert n is literal 0
    {
        let Value::Literal(literal) = &*n.borrow() else {
            panic!();
        };

        let Literal::Int(n) = &*literal.borrow() else {
            panic!();
        };

        assert_eq!(u64::try_from(n).unwrap(), 0);
    }

    // assign bit to destination
    *statement.borrow_mut() = Statement::Copy {
        expression: destination.unwrap(),
        value: bit,
    };

    false
}

pub fn assert_handler(
    _ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) -> bool {
    // get block containing statement
    // split block into first and second
    // make first conditional on arguments[0].clone(), trapping on
    // arguments[1].clone()

    let (block, idx) = function
        .entry_block
        .find_statement(statement.clone())
        .unwrap();

    let statements = block.statements();
    let terminator = block.terminator();

    let first_statements = statements[..idx].to_owned();
    let second_statements = statements[idx + 1..].to_owned();

    let (value, str) = {
        let Statement::FunctionCall { arguments, .. } = &*statement.borrow() else {
            panic!();
        };

        (arguments[0].clone(), arguments[1].clone())
    };

    // second block has the rest of the statements, and the original terminator
    let second = ControlFlowBlock::new();
    second.set_statements(second_statements);
    second.set_terminator(terminator);

    let trap_block = ControlFlowBlock::new();
    trap_block.set_statements(vec![
        Statement::Comment(format!("{:?}", str).into()).into(),
        Statement::FunctionCall {
            expression: None,
            name: "trap".into(),
            arguments: vec![],
        }
        .into(),
    ]);
    // never reached but required to maintain common block (should maybe jump to end
    // block?)
    trap_block.set_terminator(Terminator::Unconditional {
        target: second.clone(),
    });

    block.set_statements(first_statements);
    // done in this order to avoid borealis/src/genc/codegen/functions.rs:580
    block.set_terminator(Terminator::Conditional {
        condition: Value::Operation(Operation::Not(value)),
        target: trap_block,
        fallthrough: second,
    });

    true
}
