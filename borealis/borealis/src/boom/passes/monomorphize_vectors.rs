//! Monomorphize vectors (not bitvectors)
//!
//! If a register is copied to a local var, and the register has a known length,
//! change the local var to also be known length
//!
//! Not a great heuristic, possible bugs if there are multiple copies, or ???

use {
    crate::boom::passes::{any::AnyExt, Pass},
    common::boom::{control_flow::ControlFlowBlock, Ast, Expression, Statement, Type, Value},
    common::shared::Shared,
    common::HashMap,
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
            .values()
            .map(|def| monomorphize_vectors(ast.clone(), def.entry_block.clone()))
            .any()
    }
}

fn monomorphize_vectors(_: Shared<Ast>, entry_block: ControlFlowBlock) -> bool {
    let mut type_declarations = HashMap::default();

    for s in entry_block.iter().flat_map(|b| b.statements()) {
        match &*s.get() {
            Statement::VariableDeclaration { name, typ } => {
                if let Type::Vector { .. } = &*typ.get() {
                    type_declarations.insert(*name, s.clone());
                } else if let Type::FixedVector { .. } = &*typ.get() {
                    type_declarations.insert(*name, s.clone());
                }
            }
            // only consider copies into identifiers
            Statement::Copy {
                expression: Expression::Identifier(destination),
                value,
            } => {
                // If there is a destination type declaration...
                let Some(destination_type_decl) = type_declarations.get(destination) else {
                    continue;
                };

                // And, the source is an identifier..
                let Value::Identifier(source) = &*value.get() else {
                    continue;
                };

                // And, the source has a type declaration...
                let Some(source_type_decl) = type_declarations.get(source) else {
                    continue;
                };

                // And, if the destination has a type declaration that is a vector...
                let Statement::VariableDeclaration {
                    typ: destination_type,
                    ..
                } = &*destination_type_decl.get_mut()
                else {
                    continue;
                };

                let Type::Vector { .. } = &*destination_type.get() else {
                    continue;
                };

                // And, if the source has a type declaration that is a fixed vector...
                let Statement::VariableDeclaration {
                    typ: source_type, ..
                } = &*source_type_decl.get()
                else {
                    continue;
                };

                let Type::FixedVector { .. } = &*source_type.get() else {
                    continue;
                };

                // Then maybe, just maybe, we can do something.

                // Replace the destination type declaration with the source type declaration
                //*destination_type_decl.get_mut() = source_type_decl.get().clone();
                //return true;

                // if let Some(destination_type_decl) = type_declarations.get(destination) {
                //
                //     if let Value::Identifier(source) = &*value.get() {

                //
                //     if let Statement::TypeDeclaration { typ: Type::Vector { .. }, .. } = &*destination_type_decl.get() {
                //         // And, if the source has a type declaration that is a fixed vector...
                //         if let Statement::TypeDeclaration { typ: Type::FixedVector { .. }, .. } = &*

                //     }
                // }

                /*if let Type::Vector { .. } = &*original_type.get() {
                    // get type of value
                    if let Value::Identifier(source) = &*value.get() {
                        if let Some(source_type) = types.get(source) {
                            if let Type::FixedVector { length, .. } = &*source_type.get() {
                                *original_type.get_mut() = source_type.get().clone();
                                changed = true;
                                return;
                            }
                        } else if let Some(reg_type) = ast.get().registers.get(source) {
                            // assert element_types are the same
                            // replace original type with that type
                            *original_type.get_mut() = reg_type.0.get().clone();
                            changed = true;
                            return;
                        }
                    }
                }*/
            }
            _ => {}
        }
    }

    // look for copies into vectors of unknown length
    // change type declarations

    false
}
