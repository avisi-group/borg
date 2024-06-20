use {
    crate::rudder::{Statement, StatementKind},
    std::{borrow::Borrow, cmp::max},
};

// ordering derives are in discriminant order!
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ValueClass {
    None,
    Constant,
    Static,
    Dynamic,
}

impl Statement {
    pub fn classify(&self) -> ValueClass {
        match self.kind() {
            StatementKind::Constant { .. } | StatementKind::Undefined => ValueClass::Constant,

            // read value is dynamic
            StatementKind::ReadRegister { .. }
            | StatementKind::ReadMemory { .. }
            | StatementKind::ReadPc => ValueClass::Dynamic,

            // complicated! todo: be more precise here
            StatementKind::ReadVariable { .. } | StatementKind::WriteVariable { .. } => {
                ValueClass::Dynamic
            }

            StatementKind::WriteRegister { .. }
            | StatementKind::WriteMemory { .. }
            | StatementKind::Jump { .. }
            | StatementKind::Branch { .. }
            | StatementKind::Return { .. }
            | StatementKind::WritePc { .. } => ValueClass::None,

            StatementKind::PhiNode { .. } => todo!(),

            // todo: fix panic when this is correctly changed to valueclass::none
            StatementKind::Panic(_) => ValueClass::Static,

            // todo: remove me when driver is implemented
            StatementKind::PrintChar(_) => ValueClass::Static,

            // classify fails to terminate when optimizing rudder when enabled
            StatementKind::CreateBits { .. } => ValueClass::Dynamic,

            //     StatementKind::CreateSum { .. } => ValueClass::Dynamic,
            _ => classify(self.child_statements()),
        }
        // // map of statements to their value class
        // let mut classes = HashMap::default();

        // // remaining statements to classify
        // let mut remaining = vec![self.clone()];

        // while let Some(statement) = remaining.pop() {
        //     if classes.contains_key(&statement) {
        //         continue;
        //     }

        //     let class = match self.kind() {
        //         StatementKind::Constant { .. } | StatementKind::Undefined =>
        // {             Some(ValueClass::Constant)
        //         }

        //         StatementKind::ReadRegister { .. }
        //         | StatementKind::ReadMemory { .. }
        //         | StatementKind::ReadPc => Some(ValueClass::Dynamic),
        //         // complicated! todo: be more precise here
        //         StatementKind::ReadVariable { .. } |
        // StatementKind::WriteVariable { .. } => {
        // Some(ValueClass::Dynamic)         }

        //         StatementKind::WriteRegister { .. }
        //         | StatementKind::WriteMemory { .. }
        //         | StatementKind::Jump { .. }
        //         | StatementKind::Branch { .. }
        //         | StatementKind::Return { .. }
        //         | StatementKind::WritePc { .. } => Some(ValueClass::None),

        //         StatementKind::PhiNode { .. } => todo!(),

        //         // todo: fix panic when this is correctly changed to
        // valueclass::none         StatementKind::Panic(_) =>
        // Some(ValueClass::Static),         StatementKind::PrintChar(_)
        // => Some(ValueClass::Static),

        //         _ => None,
        //     };

        //     if let Some(class) = class {
        //         classes.insert(statement, class);
        //     } else {
        //         for statement in statement.child_statements() {
        //             if !classes.contains_key(&statement) {
        //                 remaining.push(statement)
        //             }
        //         }
        //     }
        // }

        // let value_classes =
        // classes.values().cloned().collect::<HashSet<_>>();

        // if value_classes.contains(&ValueClass::Dynamic) {
        //     ValueClass::Dynamic
        // } else if value_classes.contains(&ValueClass::Static) {
        //     ValueClass::Static
        // } else {
        //     ValueClass::Constant
        // }
    }
}

fn classify<I: IntoIterator<Item = S>, S: Borrow<Statement>>(iter: I) -> ValueClass {
    max_class(iter.into_iter().map(|s| s.borrow().classify()))
}

///
fn max_class<I: IntoIterator<Item = ValueClass>>(iter: I) -> ValueClass {
    let mut highest_class = ValueClass::None;

    for class in iter {
        match class {
            // any dynamic means it must be dynamic so can bail early
            ValueClass::Dynamic => return ValueClass::Dynamic,
            c => highest_class = max(c, highest_class),
        }
    }

    highest_class
}

#[cfg(test)]
mod tests {
    use crate::rudder::value_class::{max_class, ValueClass};

    #[test]
    fn max_class_dynamic_early() {
        struct Iter {
            state: u8,
        };
        impl Iterator for Iter {
            type Item = ValueClass;

            fn next(&mut self) -> Option<Self::Item> {
                let class = match self.state {
                    0 => ValueClass::Static,
                    1 => ValueClass::Dynamic,
                    _ => panic!("if classify returns early this will never be hit"),
                };

                self.state += 1;

                Some(class)
            }
        }

        assert_eq!(ValueClass::Dynamic, max_class(Iter { state: 0 }));
    }

    #[test]
    fn max_class_static() {
        assert_eq!(
            ValueClass::Static,
            max_class([ValueClass::None, ValueClass::Static, ValueClass::Constant])
        );
    }

    #[test]
    fn max_class_constant() {
        assert_eq!(
            ValueClass::Constant,
            max_class([
                ValueClass::None,
                ValueClass::Constant,
                ValueClass::Constant,
                ValueClass::None
            ])
        );
    }
}
