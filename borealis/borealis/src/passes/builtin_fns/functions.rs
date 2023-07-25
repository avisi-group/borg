use {
    crate::{
        boom::{Ast, FunctionDefinition, Statement},
        passes::builtin_fns::HandlerFunction,
    },
    common::{intern::InternedString, HashMap},
    once_cell::sync::Lazy,
    std::{cell::RefCell, rc::Rc},
};

pub const HANDLERS: Lazy<HashMap<InternedString, HandlerFunction>> = Lazy::new(|| {
    let mappings = [
        // this is a GenC builtin function so can be left as a noop here (technically not needed
        // but just being explicit)
        ("trap", noop as HandlerFunction),
        // requires bitvector conversion logic
        ("UInt", uint_handler),
        // we represent integers as u64s so these can be removed
        ("pcnt_i___pcnt_i64", replace_with_copy),
        ("pcnt_i64___pcnt_i", replace_with_copy),
        //
        ("slice", noop),
        ("Zeros", noop),
        ("undefined_bitvector", noop),
        ("bitvector_access_A", noop),
        ("eq_vec", noop),
        ("not_vec", noop),
        ("bitvector_concat", noop),
        ("ZeroExtend__0", noop),
        // non addsub_immediate functions below here
        ("undefined_bool", noop),
        ("undefined_int", noop),
        ("undefined_real", noop),
        ("undefined_string", noop),
        ("undefined_unit", noop),
        ("ediv_int", noop),
        ("not_bool", noop),
        ("eq_int", noop),
        ("gteq_int", noop),
        ("lteq_int", noop),
        ("eq_bool", noop),
        ("add_atom", noop),
        ("sub_atom", noop),
        ("negate_atom", noop),
        ("mult_atom", noop),
        ("neq_vec", noop),
        ("bitvector_length", noop),
        ("raw_GetSlice_int", noop),
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
        ("bitvector_access_B", noop),
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
        ("SInt", noop),
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

    HashMap::from_iter(mappings)
});

pub fn noop(
    _ast: Rc<RefCell<Ast>>,
    _function: FunctionDefinition,
    _statement: Rc<RefCell<Statement>>,
) {
}

/// UInt converts a bitvector into an int
pub fn uint_handler(
    ast: Rc<RefCell<Ast>>,
    function: FunctionDefinition,
    statement: Rc<RefCell<Statement>>,
) {
    // TODO: this will break when I properly implement bitvectors
    replace_with_copy(ast, function, statement)
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
