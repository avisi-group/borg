use {
    crate::boom::{
        control_flow::ControlFlowBlock, FunctionDefinition, FunctionSignature, Parameter, Size,
        Type,
    },
    common::intern::InternedString,
    std::{cell::RefCell, rc::Rc},
};

pub fn builtin_fns() -> Vec<(InternedString, FunctionDefinition)> {
    vec![]
}
