use {
    crate::boom::{
        control_flow::ControlFlowBlock, FunctionDefinition, FunctionSignature, Parameter, Type,
    },
    common::intern::InternedString,
    std::{cell::RefCell, rc::Rc},
};

pub fn builtin_fns() -> Vec<(InternedString, FunctionDefinition)> {
    vec![
        (
            "ReservedValue".into(),
            FunctionDefinition {
                signature: FunctionSignature {
                    name: "ReservedValue".into(),
                    parameters: Rc::new(RefCell::new(vec![Parameter {
                        name: "ignored".into(),
                        typ: Rc::new(RefCell::new(Type::Unit)),
                        is_ref: false,
                    }])),
                    return_type: Rc::new(RefCell::new(Type::Unit)),
                },
                entry_block: ControlFlowBlock::new(),
            },
        ),
        (
            "u__PostDecode".into(),
            FunctionDefinition {
                signature: FunctionSignature {
                    name: "u__PostDecode".into(),
                    parameters: Rc::new(RefCell::new(vec![Parameter {
                        name: "ignored".into(),
                        typ: Rc::new(RefCell::new(Type::Unit)),
                        is_ref: false,
                    }])),
                    return_type: Rc::new(RefCell::new(Type::Unit)),
                },
                entry_block: ControlFlowBlock::new(),
            },
        ),
    ]
}
