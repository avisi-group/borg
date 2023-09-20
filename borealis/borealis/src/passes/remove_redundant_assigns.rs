//! Remove redundant assignments to intermediate local variables

use {
    crate::{
        boom::{
            control_flow::Terminator,
            visitor::{Visitor, Walkable},
            Ast, Expression, FunctionDefinition, Statement, Value,
        },
        passes::{any::AnyExt, Pass},
    },
    common::{intern::InternedString, HashMap},
    std::{cell::RefCell, rc::Rc},
};

/// Remove redundant assignments to intermediate local variables
pub struct RemoveRedundantAssigns {}

impl RemoveRedundantAssigns {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::new(Self {})
    }
}

impl Pass for RemoveRedundantAssigns {
    fn name(&self) -> &'static str {
        "RemoveRedundantAssigns"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Rc<RefCell<Ast>>) -> bool {
        ast.borrow().functions.values().map(process_fn).any();
        false
    }
}

fn process_fn(fn_def: &FunctionDefinition) -> bool {
    // number of times each ident was assigned to
    let mut assignments_count = HashMap::default();
    // last value assigned to each ident
    let mut assignments_value = HashMap::default();

    fn_def
        .entry_block
        .iter()
        .map(|b| b.statements())
        .flatten()
        .for_each(|statement| {
            match &*statement.borrow() {
                Statement::Copy { expression, value } => {
                    if let Expression::Identifier(dest) = expression {
                        // increment assignment counter
                        match assignments_count.get_mut(dest) {
                            Some(count) => {
                                *count += 1;
                            }
                            None => {
                                assignments_count.insert(*dest, 1usize);
                            }
                        }

                        // TODO: maybe make this work with all values not just idents?
                        if let Value::Identifier(source) = *value.borrow() {
                            // only perform one level/step of folding per execution
                            if !assignments_value.contains_key(&source) {
                                assignments_value.insert(*dest, source);
                            }
                        }
                    }
                }
                Statement::FunctionCall { expression, .. } => {
                    if let Some(Expression::Identifier(dest)) = expression {
                        // increment assignment counter
                        match assignments_count.get_mut(dest) {
                            Some(count) => {
                                *count += 1;
                            }
                            None => {
                                assignments_count.insert(*dest, 1);
                            }
                        }
                    }
                }
                _ => (),
            }
        });

    log::trace!(
        "{}: {:?} {:?}",
        fn_def.signature.name,
        assignments_count,
        assignments_value
    );

    // number of times each ident was used
    let uses_count = UseCounter::run(fn_def);

    let redundant = assignments_count
        .into_iter()
        // only remove variables that are assigned to once
        .filter(|(_, count)| *count == 1)
        // only remove variables that are used once or less
        .filter(|(dest, _)| uses_count.get(dest).copied().unwrap_or(0) < 2)
        // never remove return values (would break tuple returns)
        .filter(|(dest, _)| !dest.as_ref().starts_with("return_value"))
        .filter_map(|(dest, _)| {
            assignments_value
                .get(&dest)
                .cloned()
                .and_then(|source| Some((dest, source)))
        })
        .collect::<HashMap<_, _>>();

    log::trace!("{}: {:?}", fn_def.signature.name, redundant);

    fn_def.entry_block.iter().for_each(|block| {
        // remove declarations and copies into redundant variables
        let new_statements = block
            .statements()
            .into_iter()
            .filter_map(|statement| match &*statement.borrow() {
                Statement::TypeDeclaration { name, .. } => {
                    if redundant.contains_key(name) {
                        None
                    } else {
                        Some(statement.clone())
                    }
                }
                Statement::Copy { expression, .. } => {
                    if let Expression::Identifier(ident) = expression {
                        if redundant.contains_key(ident) {
                            return None;
                        }
                    }

                    Some(statement.clone())
                }
                _ => Some(statement.clone()),
            })
            .collect();

        block.set_statements(new_statements);

        if let Terminator::Conditional {
            condition,
            target,
            fallthrough,
        } = block.terminator()
        {
            let condition_ref = Rc::new(RefCell::new(condition));

            IdentReplacer {
                mapping: redundant.clone(),
            }
            .visit_value(condition_ref.clone());

            let conditon_borrowed = condition_ref.borrow();
            block.set_terminator(Terminator::Conditional {
                condition: (*conditon_borrowed).clone(),
                target,
                fallthrough,
            })
        }
    });

    // replace all redundant identifiers
    IdentReplacer { mapping: redundant }.visit_function_definition(fn_def);

    false
}

struct IdentReplacer {
    mapping: HashMap<InternedString, InternedString>,
}

impl Visitor for IdentReplacer {
    fn visit_value(&mut self, node: Rc<RefCell<Value>>) {
        let replacement = if let Value::Identifier(ident) = &*node.borrow() {
            self.mapping.get(ident).cloned()
        } else {
            None
        };

        if let Some(replacement) = replacement {
            log::trace!("replacing {node:?} with {replacement}");
            *node.borrow_mut() = Value::Identifier(replacement);
        }

        node.borrow().walk(self);
    }

    // fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
    //     if let Statement::FunctionCall { expression, .. } = &mut
    // *node.borrow_mut() {         let replacement = if let
    // Some(Expression::Identifier(ident)) = expression {
    // self.mapping.get(ident).cloned()         } else {
    //             None
    //         };

    //         if let Some(replacement) = replacement {
    //             log::trace!("replacing {expression:?} with {replacement}");
    //             *expression = Some(Expression::Identifier(replacement));
    //         }
    //     }

    //     node.borrow().walk(self);
    // }
}

struct UseCounter {
    mapping: HashMap<InternedString, usize>,
}

impl UseCounter {
    fn run(fn_def: &FunctionDefinition) -> HashMap<InternedString, usize> {
        let mut celf = Self {
            mapping: HashMap::default(),
        };

        celf.visit_function_definition(fn_def);

        celf.mapping
    }
}

impl Visitor for UseCounter {
    fn visit_value(&mut self, node: Rc<RefCell<Value>>) {
        if let Value::Identifier(ident) = &*node.borrow() {
            match self.mapping.get_mut(ident) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    self.mapping.insert(*ident, 1);
                }
            }
        }

        node.borrow().walk(self);
    }

    // fn visit_statement(&mut self, node: Rc<RefCell<Statement>>) {
    //     if let Statement::FunctionCall { expression, .. } = &mut
    // *node.borrow_mut() {         let replacement = if let
    // Some(Expression::Identifier(ident)) = expression {
    // self.mapping.get(ident).cloned()         } else {
    //             None
    //         };

    //         if let Some(replacement) = replacement {
    //             log::trace!("replacing {expression:?} with {replacement}");
    //             *expression = Some(Expression::Identifier(replacement));
    //         }
    //     }

    //     node.borrow().walk(self);
    // }
}
