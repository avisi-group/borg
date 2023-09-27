//! Remove redundant assignments to intermediate local variables
//!
//! TODO: remove never used variables

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
        ast.borrow().functions.values().map(process_fn).any()
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
        .flat_map(|b| b.statements())
        .for_each(|statement| {
            match &*statement.borrow() {
                Statement::Copy {
                    expression: Expression::Identifier(dest),
                    value,
                } => {
                    // increment assignment counter
                    match assignments_count.get_mut(dest) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            assignments_count.insert(*dest, 1usize);
                        }
                    }

                    match &*value.borrow() {
                        Value::Identifier(source) => {
                            // only perform one level/step of folding per execution
                            if !assignments_value.contains_key(source) {
                                assignments_value.insert(*dest, value.clone());
                            }
                        }
                        Value::Literal(_) => {
                            assignments_value.insert(*dest, value.clone());
                        }
                        _ => (),
                    }
                }
                Statement::FunctionCall {
                    expression: Some(Expression::Identifier(dest)),
                    ..
                } => {
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
                _ => (),
            }
        });

    log::trace!(
        "{}: {:?} {:?}",
        fn_def.signature.name,
        assignments_count,
        assignments_value
    );

    let redundant = assignments_count
        .into_iter()
        // only remove variables that are assigned to once
        .filter(|(_, count)| *count == 1)
        // never remove return values (would break tuple returns)
        .filter(|(dest, _)| !dest.as_ref().starts_with("return_value"))
        // never remove exception values (TODO probably can do this)
        .filter(|(dest, _)| dest.as_ref() != "exception")
        .filter_map(|(dest, _)| {
            assignments_value
                .get(&dest)
                .cloned()
                .map(|source| (dest, source))
        })
        .collect::<HashMap<_, _>>();

    if redundant.is_empty() {
        return false;
    }

    log::trace!("{}: beepboop {:?}", fn_def.signature.name, redundant);

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

    true
}

struct IdentReplacer {
    mapping: HashMap<InternedString, Rc<RefCell<Value>>>,
}

impl Visitor for IdentReplacer {
    fn visit_value(&mut self, node: Rc<RefCell<Value>>) {
        let replacement = if let Value::Identifier(ident) = &*node.borrow() {
            self.mapping.get(ident).cloned()
        } else {
            None
        };

        if let Some(replacement) = replacement {
            log::trace!("replacing {node:?} with {replacement:?}");
            *node.borrow_mut() = replacement.borrow().clone();
        }

        node.borrow().walk(self);
    }
}
