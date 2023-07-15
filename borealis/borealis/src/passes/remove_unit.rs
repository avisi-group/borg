//! Removes unit local variables and assignments, void parameters from function
//! definitions, and void arguments from function calls

use {
    crate::{
        boom::{
            control_flow::ControlFlowBlock,
            visitor::{Visitor, Walkable},
            Ast, Expression, Literal, Statement, Type, Value,
        },
        passes::{any::AnyExt, Pass},
    },
    common::intern::InternedString,
    log::trace,
    std::{cell::RefCell, collections::HashSet, rc::Rc},
};

pub struct RemoveUnits {
    ast: Rc<RefCell<Ast>>,
    did_change: bool,
    visited_blocks: HashSet<ControlFlowBlock>,
    modified_fns: HashSet<InternedString>,
    deleted_unit_vars: HashSet<InternedString>,
}

impl Pass for RemoveUnits {
    fn name(&self) -> &'static str {
        "RemoveUnit"
    }

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow()
            .functions
            .values()
            .map(|def| {
                self.did_change = false;
                self.visited_blocks = HashSet::new();
                self.deleted_unit_vars = HashSet::new();

                if *def.signature.return_type.borrow() == Type::Unit {
                    self.deleted_unit_vars.insert("return_type".into());
                    self.deleted_unit_vars.insert("return".into());
                }

                self.visit_function_definition(def);

                self.did_change
            })
            .any()
    }
}

impl Visitor for RemoveUnits {
    // visit function signature, then every callsite

    fn visit_control_flow_block(&mut self, block: &ControlFlowBlock) {
        if self.is_block_visited(block) {
            return;
        }
        self.set_block_visited(block);

        let statements = block
            .statements()
            .into_iter()
            .filter_map(|statement| self.statement_filter(statement))
            .collect();

        block.set_statements(statements);

        block.walk(self);
    }

    fn is_block_visited(&mut self, block: &ControlFlowBlock) -> bool {
        self.visited_blocks.contains(block)
    }

    fn set_block_visited(&mut self, block: &ControlFlowBlock) {
        self.visited_blocks.insert(block.clone());
    }
}

impl RemoveUnits {
    pub fn new_boxed(ast: Rc<RefCell<Ast>>) -> Box<dyn Pass> {
        Box::new(Self {
            ast,
            did_change: false,
            visited_blocks: HashSet::new(),
            modified_fns: HashSet::new(),
            deleted_unit_vars: HashSet::new(),
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
                if arguments.iter().any(is_unit_value) {
                    if !self.modified_fns.contains(name) {
                        trace!("function {name:?} not currently modified but had unit value");
                    }
                    arguments.retain(|value| !is_unit_value(value));
                }

                Some(statement_cloned)
            }

            _ => Some(statement_cloned),
        }
    }
}

fn is_unit_value(value: &Value) -> bool {
    let Value::Literal(literal) = value else {
        return false;
    };

    let Literal::Unit = &*literal.borrow() else {
        return false;
    };

    true
}
