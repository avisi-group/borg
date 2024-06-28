use {
    crate::rudder::statement::StatementKind,
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

pub fn classify_kind(kind: &StatementKind) -> ValueClass {
    match kind {
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

        _ => max_class(kind.children().into_iter().map(|s| s.borrow().class())),
    }
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
    use crate::rudder::statement::value_class::{max_class, ValueClass};

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
