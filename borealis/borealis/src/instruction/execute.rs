//! Instruction execute behaviour

use crate::boom::{
    control_flow::ControlFlowBlock, FunctionDefinition, FunctionSignature, NamedType,
};

/// Emit a BOOM function as a GenC function
pub fn boom_fn_to_genc(
    FunctionDefinition {
        signature,
        entry_block,
    }: &FunctionDefinition,
) -> String {
    let FunctionSignature {
        name,
        parameters,
        return_type,
    } = signature;

    let body = emit_body(&entry_block);

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

fn emit_body(_entry_block: &ControlFlowBlock) -> String {
    "println(\"hello\")".to_owned()
}
