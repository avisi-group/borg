//! Monomorphize vectors (not bitvectors)
//!
//! If a register is copied to a local var, and the register has a known length,
//! change the local var to also be known length
//!
//! Not a great heuristic, possible bugs if there are multiple copies, or ???

use {
    crate::boom::{
        control_flow::ControlFlowBlock,
        passes::{any::AnyExt, Pass},
        Ast, Expression, Statement, Type, Value,
    },
    common::{shared::Shared, HashMap},
};

#[derive(Debug, Default)]
pub struct MonomorphizeVectors;

impl MonomorphizeVectors {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for MonomorphizeVectors {
    fn name(&self) -> &'static str {
        "MonomorphizeVectors"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get()
            .functions
            .iter()
            .map(|(_, def)| monomorphize_vectors(ast.clone(), def.entry_block.clone()))
            .any()
    }
}

fn monomorphize_vectors(ast: Shared<Ast>, entry_block: ControlFlowBlock) -> bool {
    let mut types = HashMap::default();

    entry_block
        .iter()
        .map(|b| b.statements())
        .flatten()
        .for_each(|s| match &*s.get() {
            Statement::TypeDeclaration { name, typ } => {
                if let Type::Vector { .. } = &*typ.get() {
                    types.insert(*name, typ.clone());
                }
            }
            Statement::Copy { expression, value } => {
                // only consider copies into identifiers
                if let Expression::Identifier(destination) = expression {
                    // only consider if that identifier is a vector
                    if let Some(original_type) = types.get(destination) {
                        // get type of value
                        if let Value::Identifier(source) = &*value.get() {
                            if let Some(reg_type) = ast.get().registers.get(source) {
                                // assert element_types are the same
                                // replace original type with that type
                                *original_type.get_mut() = reg_type.0.get().clone();
                            }
                        }
                    }
                }
            }
            _ => {}
        });

    // look for copies into vectors of unknown length
    // change type declarations

    false
}
