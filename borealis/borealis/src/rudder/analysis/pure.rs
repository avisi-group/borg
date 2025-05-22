use {
    crate::{TREAT_PANICS_AS_PURE_DANGEROUS_UNSAFE, rudder::opt::INTRINSICS},
    common::{
        hashmap::{HashMap, HashSet},
        intern::InternedString,
        rudder::{Model, statement::Statement},
    },
};

pub struct PurityAnalysis {
    functions: HashMap<InternedString, bool>,
}

impl PurityAnalysis {
    pub fn new(model: &Model) -> Self {
        let mut celph = Self {
            functions: HashMap::default(),
        };

        // assume impure
        celph.functions.extend(
            INTRINSICS
                .iter()
                .map(|s| InternedString::from_static(s))
                .map(|s| (s, false)),
        );

        let mut seen = HashSet::default();

        for f in model.functions().keys() {
            celph.determine_purity_recursive(model, &mut seen, *f);
        }

        celph
    }

    pub fn is_pure(&self, function: InternedString) -> bool {
        *self.functions.get(&function).unwrap()
    }

    fn determine_purity_recursive(
        &mut self,
        model: &Model,
        seen: &mut HashSet<InternedString>,
        name: InternedString,
    ) -> bool {
        if let Some(is_pure) = self.functions.get(&name) {
            return *is_pure;
        }

        seen.insert(name);
        log::debug!("determining purity of {name:?}");

        let function = model.functions().get(&name).unwrap();

        let is_pure = function
            .block_iter()
            .flat_map(|b| {
                let block = b.get(function.arena());
                block.statements().iter().map(|s| (s, block.arena()))
            })
            .map(|(s, arena)| s.get(arena))
            .all(|s| match s {
                Statement::Call { target, .. } => {
                    if seen.contains(target) {
                        log::debug!("impure due to recursion of {target:?}");
                        false
                    } else {
                        let res = self.determine_purity_recursive(model, seen, *target);

                        if !res {
                            log::debug!("impure due to call to {target:?}");
                        }

                        res
                    }
                }
                Statement::WriteMemory { .. }
                | Statement::WritePc { .. }
                | Statement::WriteRegister { .. }
                | Statement::ReadMemory { .. } => {
                    log::debug!("impure due to {s:?}");

                    false
                }

                Statement::Panic(_) | Statement::Assert { .. } => {
                    TREAT_PANICS_AS_PURE_DANGEROUS_UNSAFE
                }
                _ => true,
            });

        self.functions.insert(name, is_pure);
        seen.remove(&name);
        log::debug!("determined purity of {name:?} = {is_pure}");

        is_pure
    }
}
