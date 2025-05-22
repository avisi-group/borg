use {
    crate::rudder::{analysis, opt::OptimizationContext},
    common::rudder::{Model, block::Block},
};

pub fn run(ctx: &OptimizationContext, model: &mut Model) -> bool {
    let mut changed = false;

    let mut dead_parameters = vec![];

    for (_, f) in model.functions_mut() {
        let dfa = analysis::dfa::SymbolUseAnalysis::new(f);

        for (i, sym) in f.parameters().iter().enumerate().rev() {
            // rev because we want to remove parameters in reverse order to not mess up the
            // indices
            if dfa.is_symbol_dead(&sym) {
                dead_parameters.push((f.name(), i));
                f.remove_parameter(&sym);
            }
        }
    }

    //     fix call sites
    for (_, function) in model.functions_mut() {
        function
            .block_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|b| {
                let Block {
                    statements,
                    statement_arena,
                } = b.get_mut(function.arena_mut());

                for s in statements {
                    if let common::rudder::statement::Statement::Call { target, args, .. } =
                        s.get_mut(statement_arena)
                    {
                        dead_parameters
                            .iter()
                            .filter(|(name, _)| *name == *target)
                            .for_each(|(_, index)| {
                                args.remove(*index);
                                changed = true;
                            });
                    }
                }
            });
    }

    changed
}
