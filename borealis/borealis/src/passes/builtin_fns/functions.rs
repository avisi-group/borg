use {
    crate::boom::{Ast, Statement},
    phf::{phf_map, Map},
};

pub static HANDLERS: Map<&str, fn(&Ast, &Statement)> = phf_map! {
    "undefined_bool" => noop,
    "undefined_int" => noop,
    "undefined_real" => noop,
    "undefined_string" => noop,
    "undefined_unit" => noop,
    "ediv_int" => noop,
    "not_bool" => noop,
    "eq_int" => noop,
    "eq_bool" => noop,
    "add_atom" => noop,
    "sub_atom" => noop,
    "negate_atom" => noop,
    "mult_atom" => noop,
    "eq_vec" => noop,
    "bitvector_length" => noop,
    "bitvector_concat" => noop,
    "__raw_GetSlice_int" => noop,
    "DecStr" => noop,
    "asl_prerr_string" => noop,
    "replicate_bits" => noop,
    "slice" => noop,
    "Error_Implementation_Defined" => noop,
    "Error_ExceptionTaken" => noop,
    "append_str" => noop,
    "__GetVerbosity" => noop,
    "___WriteRAM_bool" => noop,
    "%i64->%i" => noop,
    "cons" => noop,
    "sail_assert" => noop,
};

fn noop(_ast: &Ast, _node: &Statement) {}
