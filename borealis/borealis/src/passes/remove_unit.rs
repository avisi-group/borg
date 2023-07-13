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
    did_change: bool,
    visited_blocks: HashSet<ControlFlowBlock>,
    modified_fns: HashSet<InternedString>,
}

impl RemoveUnits {
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {
            did_change: false,
            visited_blocks: HashSet::new(),
            modified_fns: HashSet::new(),
        })
    }
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

        let mut deleted_unit_vars = HashSet::new();

        let statements = block
            .statements()
            .into_iter()
            .filter_map(|statement| {
                statement_filter(&mut deleted_unit_vars, &mut self.modified_fns, statement)
            })
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

fn statement_filter(
    deleted_unit_vars: &mut HashSet<InternedString>,
    modified_fns: &mut HashSet<InternedString>,
    statement: Rc<RefCell<Statement>>,
) -> Option<Rc<RefCell<Statement>>> {
    let statement_cloned = statement.clone();

    match &mut *statement.borrow_mut() {
        Statement::TypeDeclaration { name, typ } => {
            if let Type::Unit = &*typ.borrow() {
                deleted_unit_vars.insert(*name);
                return None;
            } else {
                return Some(statement_cloned);
            }
        }

        Statement::Copy {
            expression,
            ref value,
        } => {
            if is_unit_value(&value) {
                // statement is an assignment of a unit literal

                if let Expression::Identifier(ident) = expression {
                    if !deleted_unit_vars.contains(&ident) {
                        trace!("removing assignment of unit to {ident:?} before type definitin was removed")
                    }

                    None
                } else {
                    trace!(
                        "assignment of a unit literal to a non-identifier expression: {expression:?}"
                    );
                    Some(statement_cloned)
                }
            } else {
                Some(statement_cloned)
            }
        }

        Statement::FunctionCall {
            name, arguments, ..
        } => {
            // if any of the arguments are unit values, remove them
            if arguments.iter().any(is_unit_value) {
                if !modified_fns.contains(&name) {
                    trace!("function {name:?} not currently modified but had unit value");
                }
                arguments.retain(|value| !is_unit_value(value));
            }

            Some(statement_cloned)
        }

        _ => Some(statement_cloned),
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
