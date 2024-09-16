use {
    crate::boom::{passes::Pass, Ast, Bit, Literal, Statement, Value},
    common::shared::Shared,
};

#[derive(Debug, Default)]
pub struct RemoveUndefinedBV;

impl RemoveUndefinedBV {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for RemoveUndefinedBV {
    fn name(&self) -> &'static str {
        "RemoveUndefinedBV"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        ast.get()
            .functions
            .iter()
            .flat_map(|(_, def)| def.entry_block.iter())
            .for_each(|b| {
                b.set_statements(
                    b.statements()
                        .into_iter()
                        .map(|s| {
                            if let Statement::FunctionCall {
                                name,
                                expression: Some(expression),
                                ..
                            } = &*(s.get())
                            {
                                if name.as_ref() == "undefined_bitvector" || name.as_ref() == "undefined_vector<b>" {
                                    Shared::new(Statement::Copy {
                                        expression: expression.clone(),
                                        value: Shared::new(Value::Literal(Shared::new(Literal::Bits(vec![Bit::Zero])))),
                                    })
                                } else {
                                    s.clone()
                                }
                            } else {
                                s.clone()
                            }
                        })
                        .collect(),
                );
            });

        false
    }
}
