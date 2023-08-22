use {
    crate::{
        boom::{
            Ast, FunctionDefinition, Literal, Operation, OperationKind, Statement, Type, Value,
        },
        passes::builtin_fns::HandlerFunction,
    },
    common::{intern::InternedString, HashMap},
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
            replace_with_infix(ast, f, s, OperationKind::Equal)
        }),
        ("eq_vec", |ast, f, s| {
            replace_with_infix(ast, f, s, OperationKind::Equal)
        }),
        ("not_vec", |ast, f, s| {
            replace_with_infix(ast, f, s, OperationKind::Complement)
        }),
        ("not_bool", |ast, f, s| {
            replace_with_infix(ast, f, s, OperationKind::Not)
        }),
        ("add_atom", |ast, f, s| {
            replace_with_infix(ast, f, s, OperationKind::Add)
        }),
        ("sub_atom", |ast, f, s| {
            replace_with_infix(ast, f, s, OperationKind::Subtract)
        }),
        // probably need to take another look at this
        ("SInt", replace_with_copy),
        ("bitvector_length", bv_length_handler),
        ("bitvector_access_B", bv_access_handler),
        ("raw_GetSlice_int", noop),
        //
        ("slice", noop),
        ("Zeros", noop),
        ("undefined_bitvector", noop),
        ("bitvector_access_A", noop),
        ("bitvector_concat", noop),
        ("ZeroExtend__0", noop),
        // non addsub_immediate functions below here
        ("undefined_bool", noop),
        ("undefined_int", noop),
        ("undefined_real", noop),
        ("undefined_string", noop),
        ("undefined_unit", noop),
        ("ediv_int", noop),
        ("gteq_int", noop),
        ("lteq_int", noop),
        ("eq_bool", noop),
        ("negate_atom", noop),
        ("mult_atom", noop),
        ("neq_vec", noop),
        ("DecStr", noop),
        ("asl_prerr_string", noop),
        ("replicate_bits", noop),
        ("Error_Implementation_Defined", noop),
        ("Error_ExceptionTaken", noop),
        ("append_str", noop),
        ("GetVerbosity", noop),
        ("WriteRAM_bool", noop),
        ("cons", noop),
        ("sail_assert", noop),
        ("Error_See", noop),
        ("update_fbits", noop),
        ("vector_access_A_B32_", noop),
        ("SetSlice_bits", noop),
        ("Error_Undefined", noop),
        ("shl8", noop),
        ("add_vec", noop),
        ("xor_vec", noop),
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
        ("shl_int", noop),
        ("or_vec", noop),
        ("vector_update_B_o_", noop),
        ("eq_anything_ESRType_pcnt__", noop),
        ("internal_pick_EMBReqTypes_pcnt__", noop),
        ("internal_pick_EImmediateOp_pcnt__", noop),
        ("and_vec", noop),
        ("undefined_vector_B64_", noop),
        ("undefined_vector_B128_", noop),
        ("undefined_vector_i_", noop),
        ("vector_update_B_B16_", noop),
        ("check_cycle_count", noop),
        ("shr_int", noop),
        ("eq_string", noop),
        ("lt_int", noop),
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

/// DEPRECATING TEMPORARILY, GENC INTRINSICS REQUIRED FOR MAX PERF BUT CURRENT
/// BOREALIS HEURISTICS NOT POWERFUL ENOUGH
// /// Add with carry handler
// pub fn add_with_carry_handler(
//     _ast: Rc<RefCell<Ast>>,
//     function: FunctionDefinition,
//     statement: Rc<RefCell<Statement>>,
// ) { // find the block containing the addwithcarry function call, and it's position // within that
//   block let (block, idx) = function .entry_block .find_statement(statement.clone()) .unwrap();

//     // get the block statements as a local mutable vec
//     let mut block_statements = block.statements();

//     // delete statement
//     block_statements.remove(idx);

//     let Statement::FunctionCall {
//         expression: Some(expression),
//         arguments,
//         ..
//     } = &*statement.borrow()
//     else {
//         panic!()
//     };

//     let Expression::Identifier(original_target) = expression else {
//         panic!()
//     };

//     // remove type declaration for result struct, store names of fields
//     let index = block_statements
//         .iter()
//         .position(|stmt| {
//             let Statement::TypeDeclaration { name, .. } = &*stmt.borrow() else {
//                 return false;
//             };

//             name == original_target
//         })
//         .unwrap();

//     let statement = block_statements.remove(index);

//     // remove type declaration
//     let (result_field, flags_field) = {
//         let Statement::TypeDeclaration { typ, .. } = &*statement.borrow() else {
//             panic!();
//         };

//         let Type::Struct { fields, .. } = &*typ.borrow() else {
//             panic!();
//         };

//         (fields[0].name, fields[1].name)
//     };

//     let id = unique_id();
//     let flags_ident = InternedString::from(format!("adc_flags{id}"));
//     let result_ident = InternedString::from(format!("adc_result{id}"));

//     block_statements.splice(
//         idx..idx,
//         [
//             // create local vars for flags and result of the same type as the removed struct
//             Rc::new(RefCell::new(Statement::TypeDeclaration {
//                 name: flags_ident,
//                 typ: Rc::new(RefCell::new(Type::FixedBits(4, false))),
//             })),
//             Rc::new(RefCell::new(Statement::TypeDeclaration {
//                 name: result_ident,
//                 typ: Rc::new(RefCell::new(Type::FixedBits(64, false))),
//             })),
//             // create two new statements,genc_adc64_flags and genc_adc64 assigned to local
//             // vars
//             Rc::new(RefCell::new(Statement::FunctionCall {
//                 expression: Some(Expression::Identifier(flags_ident)),
//                 name: "__builtin_adc64_flags".into(),
//                 arguments: arguments.clone(),
//             })),
//             Rc::new(RefCell::new(Statement::FunctionCall {
//                 expression: Some(Expression::Identifier(result_ident)),
//                 name: "__builtin_adc64".into(),
//                 arguments: arguments.clone(),
//             })),
//         ],
//     );

//     block.set_statements(block_statements);

//     // remove any Field value with the `original_target` identifier (iterating
//     // over the current block and its children) and replace with ident of local
//     // var
//     block
//         .iter()
//         .map(|b| b.statements())
//         .flatten()
//         .for_each(|stmt| {
//             let new_value = {
//                 let Statement::Copy { value, .. } = &*stmt.borrow() else {
//                     return;
//                 };

//                 let Value::Field { value, field_name } = value else {
//                     return;
//                 };

//                 let Value::Identifier(ident) = **value else {
//                     return;
//                 };

//                 if ident != *original_target {
//                     return;
//                 }

//                 if *field_name == result_field {
//                     Value::Identifier(result_ident)
//                 } else if *field_name == flags_field {
//                     Value::Identifier(flags_ident)
//                 } else {
//                     return;
//                 }
//             };

//             let Statement::Copy { value, .. } = &mut *stmt.borrow_mut() else {
//                 panic!()
//             };

//             *value = new_value;
//         });
// }

///
pub fn replace_with_infix(
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
        OperationKind::LessThan => Operation::LessThan(args[0].clone(), args[1].clone()),
        OperationKind::GreaterThan => Operation::GreaterThan(args[0].clone(), args[1].clone()),
        OperationKind::Subtract => Operation::Subtract(args[0].clone(), args[1].clone()),
        OperationKind::Add => Operation::Add(args[0].clone(), args[1].clone()),
        OperationKind::Or => Operation::Or(args[0].clone(), args[1].clone()),
        OperationKind::And => Operation::And(args[0].clone(), args[1].clone()),
        OperationKind::LeftShift => Operation::LeftShift(args[0].clone(), args[1].clone()),
        OperationKind::RightShift => Operation::RightShift(args[0].clone(), args[1].clone()),
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
