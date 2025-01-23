use {
    crate::boom::{
        control_flow::ControlFlowBlock,
        passes::{any::AnyExt, Pass},
        visitor::Visitor,
        Ast, Expression, Literal, Statement, Value,
    },
    common::{intern::InternedString, HashMap, HashSet},
    sailrs::shared::Shared,
};

#[derive(Debug, Default)]
pub struct ConstantPropogation;

impl ConstantPropogation {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for ConstantPropogation {
    fn name(&self) -> &'static str {
        "ConstantPropogation"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        let ast = ast.get();

        ast.functions
            .iter()
            .map(|(_, def)| function_constant_propogation(def.entry_block.clone()))
            .any()
    }
}

// todo: constant evaluation
fn function_constant_propogation(entry_block: ControlFlowBlock) -> bool {
    // one pass to build constants, second pass to replace them

    let constants = {
        let mut candidate_constants = HashMap::<InternedString, Literal>::default();
        let mut mutable_vars = HashSet::<InternedString>::default();

        entry_block
            .iter()
            .flat_map(|b| b.statements())
            .for_each(|s| {
                // only look at copy statements
                // EDIT: 2025-01-23 function statements ALSO write variables you idiot
                match &*s.get() {
                    Statement::Copy {
                        expression: Expression::Identifier(target),
                        value,
                    } => {
                        {
                            match (
                                candidate_constants.contains_key(target),
                                mutable_vars.contains(target),
                            ) {
                                (true, true) => {
                                    panic!(
                                        "cannot be a candidate but also a known mutable variable"
                                    )
                                }
                                (true, false) => {
                                    // we are writing (again) to a variable we thought was constant, so it
                                    // is not a constant
                                    candidate_constants.remove(target);
                                    mutable_vars.insert(*target);
                                }
                                (false, true) => {
                                    // known mutable being written to again, no-op
                                }
                                (false, false) => {
                                    // new variable written to for the first time
                                    // if it's a literal
                                    if let Value::Literal(literal) = &*value.get() {
                                        // save it as a potential constant
                                        candidate_constants.insert(*target, literal.get().clone());
                                    }
                                }
                            }
                        }
                    }
                    Statement::FunctionCall {
                        expression: Some(Expression::Identifier(target)),
                        ..
                    } => {
                        // we are writing to a variable we thought was constant, so it
                        // is not a constant
                        candidate_constants.remove(target);
                        mutable_vars.insert(*target);
                    }
                    _ => (),
                }
            });

        candidate_constants
    };

    // at this point all our candidate constants are now known to be constant
    // replace every use of those identifiers with literals

    struct ReplacerVisitor {
        constants: HashMap<InternedString, Literal>,
        did_change: bool,
    }

    impl Visitor for ReplacerVisitor {
        fn visit_value(&mut self, node: Shared<Value>) {
            let node = &mut *node.get_mut();
            if let Value::Identifier(ident) = node {
                if let Some(literal) = self.constants.get(ident) {
                    *node = Value::Literal(Shared::new(literal.clone()));
                    self.did_change = true;
                }
            }
        }
    }

    let mut visitor = ReplacerVisitor {
        constants,
        did_change: false,
    };
    visitor.visit_control_flow_block(&entry_block);

    visitor.did_change
}
