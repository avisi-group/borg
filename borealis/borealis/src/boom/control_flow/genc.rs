use crate::boom::{
    control_flow::ControlFlowBlock, FunctionDefinition, FunctionSignature, NamedType,
};

struct GenCEmitter {
    entry_block: ControlFlowBlock,
}

impl GenCEmitter {
    pub fn _new(entry_block: ControlFlowBlock) -> Self {
        Self { entry_block }
    }
}

/// Emit a BOOM function as a GenC function
pub fn _boom_fn_to_genc(FunctionDefinition { signature, .. }: &FunctionDefinition) -> String {
    let FunctionSignature {
        name,
        parameters,
        return_type,
    } = signature;

    let body = "emit_body(&entry_block);".to_owned();

    let parameters = {
        let mut s = String::new();

        let mut parameters = parameters.iter();
        if let Some(NamedType { name, typ }) = parameters.next() {
            s += &format!("{typ} {name}");
        }
        for NamedType { name, typ } in parameters {
            s += ", ";
            s += &format!("{typ} {name}");
        }

        s
    };

    format!(
        r#"{return_type} {name}({parameters}) {{
            {body}
        }}
        "#
    )
}
