use {
    crate::boom::passes::Pass,
    common::boom::{Ast, Size, Statement, Type},
    common::shared::Shared,
    common::width_helpers::signed_smallest_width_of_value,
};

#[derive(Debug, Default)]
pub struct RemoveConstantType;

impl RemoveConstantType {
    /// Create a new Pass object
    pub fn new_boxed() -> Box<dyn Pass> {
        Box::<Self>::default()
    }
}

impl Pass for RemoveConstantType {
    fn name(&self) -> &'static str {
        "RemoveConstantType"
    }

    fn reset(&mut self) {}

    fn run(&mut self, ast: Shared<Ast>) -> bool {
        let ast = ast.get();

        ast.functions
            .iter()
            .flat_map(|(_, def)| def.entry_block.iter())
            .for_each(|b| {
                b.set_statements(
                    b.statements()
                        .into_iter()
                        .map(|s| match &*(s.get()) {
                            Statement::VariableDeclaration { name, typ } => match *(typ.get()) {
                                Type::Constant(v) => Shared::new(Statement::VariableDeclaration {
                                    name: *name,
                                    typ: Shared::new(Type::Integer {
                                        size: Size::Static(
                                            signed_smallest_width_of_value(v).try_into().unwrap(),
                                        ),
                                    }),
                                }),
                                _ => s.clone(),
                            },
                            _ => s.clone(),
                        })
                        .collect(),
                );
            });

        false
    }
}
