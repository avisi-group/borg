//! Removes unit local variables and assignments, void parameters from function
//! definitions, and void arguments from function calls

use {
    crate::boom::{
        control_flow::ControlFlowBlock,
        passes::{any::AnyExt, Pass},
        visitor::{Visitor, Walkable},
        Ast, Expression, FunctionDefinition, Literal, Parameter, Statement, Type, Value,
    },
    common::{intern::InternedString, HashSet},
    log::trace,
    std::{cell::RefCell, rc::Rc},
};

/// Removes unit local variables and assignments, void parameters from function
/// definitions, and void arguments from function calls
pub struct RemoveUnits {
    ast: Rc<RefCell<Ast>>,
    did_change: bool,
    visited_blocks: HashSet<ControlFlowBlock>,
    deleted_unit_vars: HashSet<InternedString>,
}

impl Pass for RemoveUnits {
    fn name(&self) -> &'static str {
        "RemoveUnit"
    }

    fn reset(&mut self) {
        self.did_change = false;
        self.visited_blocks.clear();
        self.deleted_unit_vars.clear();
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|def| {
                self.reset();

                if let Type::Unit = *def.signature.return_type.borrow() {
                    self.deleted_unit_vars.insert("return_type".into());
                    self.deleted_unit_vars.insert("return".into());
                    self.deleted_unit_vars.insert("return_value".into());
                }

                self.visit_function_definition(def);

                self.did_change
            })
            .any()
    }
}

impl Visitor for RemoveUnits {
    fn visit_function_definition(&mut self, node: &FunctionDefinition) {
        node.signature
            .parameters
            .borrow_mut()
            // retain if not unit
            .retain(|Parameter { typ, .. }| !matches!(*typ.borrow(), Type::Unit));
        node.walk(self);
    }

    // visit function signature, then every callsite
    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        let statements = block
            .statements()
            .into_iter()
            .filter_map(|statement| self.statement_filter(statement))
            .collect();

        block.set_statements(statements);

        block.walk(self);
    }
}

impl RemoveUnits {
    /// Create a new Pass object
    pub fn new_boxed(ast: Rc<RefCell<Ast>>) -> Box<dyn Pass> {
        Box::new(Self {
            ast,
            did_change: false,
            visited_blocks: HashSet::default(),
            deleted_unit_vars: HashSet::default(),
        })
    }

    fn statement_filter(
        &mut self,
        statement: Rc<RefCell<Statement>>,
    ) -> Option<Rc<RefCell<Statement>>> {
        let statement_cloned = statement.clone();

        match &mut *statement.borrow_mut() {
            Statement::TypeDeclaration { name, typ } => {
                if let Type::Unit = &*typ.borrow() {
                    self.deleted_unit_vars.insert(*name);
                    None
                } else {
                    Some(statement_cloned)
                }
            }

            Statement::Copy { expression, .. } => {
                if let Expression::Identifier(ident) = expression {
                    // if we are assigning to a deleted unit variable, drop the assignment
                    if self.deleted_unit_vars.contains(ident) {
                        None
                    } else {
                        Some(statement_cloned)
                    }
                } else {
                    Some(statement_cloned)
                }
            }

            Statement::FunctionCall {
                name,
                arguments,
                expression,
            } => {
                // if the return type is void, make the expression None
                if let Some(func) = self.ast.borrow().functions.get(name) {
                    if let Type::Unit = *func.signature.return_type.borrow() {
                        *expression = None;
                    }
                } else {
                    trace!("call made to missing function :(");
                }

                // if any of the arguments are unit values, remove them
                if arguments.iter().cloned().any(is_unit_value) {
                    arguments.retain(|value| !is_unit_value(value.clone()));
                }

                Some(statement_cloned)
            }

            _ => Some(statement_cloned),
        }
    }
}

fn is_unit_value(value: Rc<RefCell<Value>>) -> bool {
    let Value::Literal(literal) = &*value.borrow() else {
        return false;
    };

    let Literal::Unit = &*literal.borrow() else {
        return false;
    };

    true
}
