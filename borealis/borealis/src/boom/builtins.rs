use crate::boom::{
    control_flow::ControlFlowBlock, FunctionDefinition, FunctionSignature, Parameter, Size, Type,
};
use common::intern::InternedString;
use std::{cell::RefCell, rc::Rc};

pub fn builtin_fns() -> Vec<(InternedString, FunctionDefinition)> {
    vec![]
}
