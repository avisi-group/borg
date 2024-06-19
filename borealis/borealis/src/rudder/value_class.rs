use crate::rudder::{Statement, StatementKind};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ValueClass {
    None,
    Constant,
    Static,
    Dynamic,
}

impl Statement {
    pub fn classify(&self) -> ValueClass {
        match self.kind() {
            StatementKind::Constant { .. } => ValueClass::Constant,
            StatementKind::ReadRegister { .. } => ValueClass::Dynamic,
            StatementKind::WriteRegister { .. } => ValueClass::None,
            StatementKind::ReadMemory { .. } => ValueClass::Dynamic,
            StatementKind::WriteMemory { .. } => ValueClass::None,
            StatementKind::BinaryOperation { lhs, rhs, .. } => {
                match (lhs.classify(), rhs.classify()) {
                    (ValueClass::Constant, ValueClass::Constant) => ValueClass::Constant,
                    (ValueClass::Constant, ValueClass::Static) => ValueClass::Static,
                    (ValueClass::Constant, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Static, ValueClass::Constant) => ValueClass::Static,
                    (ValueClass::Static, ValueClass::Static) => ValueClass::Static,
                    (ValueClass::Static, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Constant) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Static) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Dynamic) => ValueClass::Dynamic,
                    _ => panic!("cannot classify binary operation"),
                }
            }
            StatementKind::UnaryOperation { value, .. } => match value.classify() {
                ValueClass::Constant => ValueClass::Constant,
                ValueClass::Static => ValueClass::Static,
                ValueClass::Dynamic => ValueClass::Dynamic,
                _ => panic!("cannot classify unary operation"),
            },
            StatementKind::ShiftOperation { value, amount, .. } => {
                match (value.classify(), amount.classify()) {
                    (ValueClass::Constant, ValueClass::Constant) => ValueClass::Constant,
                    (ValueClass::Constant, ValueClass::Static) => ValueClass::Static,
                    (ValueClass::Static, ValueClass::Constant) => ValueClass::Static,
                    (ValueClass::Dynamic, ValueClass::Constant) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Static) => ValueClass::Dynamic,
                    (ValueClass::Dynamic, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Constant, ValueClass::Dynamic) => ValueClass::Dynamic,
                    (ValueClass::Static, ValueClass::Dynamic) => ValueClass::Dynamic,
                    _ => panic!("cannot classify shift operation"),
                }
            }
            StatementKind::Call { args, .. } => {
                if args.iter().any(|a| a.classify() == ValueClass::None) {
                    panic!("illegal arguments to function call");
                }

                if args.iter().any(|a| a.classify() == ValueClass::Dynamic) {
                    ValueClass::Dynamic
                } else {
                    ValueClass::Static
                }
            }
            StatementKind::Cast { value, .. } => match value.classify() {
                ValueClass::Constant => ValueClass::Constant,
                ValueClass::Static => ValueClass::Static,
                ValueClass::Dynamic => ValueClass::Dynamic,
                ValueClass::None => {
                    panic!("cannot classify cast operation {:?} in {:?}", value, self)
                }
            },
            StatementKind::Jump { .. } => ValueClass::None,
            StatementKind::Branch { .. } => ValueClass::None,
            StatementKind::PhiNode { .. } => todo!(),
            StatementKind::Return { .. } => ValueClass::None,
            StatementKind::Select {
                condition,
                true_value,
                false_value,
            } => {
                match (
                    condition.classify(),
                    true_value.classify(),
                    false_value.classify(),
                ) {
                    (ValueClass::Constant, ValueClass::Constant, ValueClass::Constant) => {
                        ValueClass::Constant
                    }
                    (ValueClass::Static, ValueClass::Static, ValueClass::Static) => {
                        ValueClass::Static
                    }
                    _ => ValueClass::Dynamic,
                }
            }
            StatementKind::Panic(_) => ValueClass::Static,
            StatementKind::PrintChar(_) => ValueClass::Static,
            StatementKind::ReadPc => ValueClass::Dynamic,
            StatementKind::WritePc { .. } => ValueClass::None,
            StatementKind::BitExtract { .. } => ValueClass::Dynamic,
            StatementKind::BitInsert { .. } => ValueClass::Dynamic,
            StatementKind::ReadVariable { .. } => ValueClass::Dynamic,
            StatementKind::WriteVariable { .. } => ValueClass::Dynamic,
            StatementKind::ReadElement { .. } => ValueClass::Dynamic,
            StatementKind::MutateElement { .. } => ValueClass::Dynamic,
            StatementKind::CreateProduct { .. } => ValueClass::Dynamic,
            StatementKind::SizeOf { .. } => ValueClass::Dynamic,
            StatementKind::Assert { .. } => ValueClass::None,
            StatementKind::BitsCast { .. } => ValueClass::Dynamic,
            StatementKind::CreateBits { .. } => ValueClass::Dynamic,
            StatementKind::CreateSum { .. } => ValueClass::Dynamic,
            StatementKind::MatchesSum { .. } => ValueClass::Dynamic,
            StatementKind::UnwrapSum { .. } => ValueClass::Dynamic,
            StatementKind::ExtractField { .. } => ValueClass::Dynamic,
            StatementKind::UpdateField { .. } => ValueClass::Dynamic,
            StatementKind::Undefined => ValueClass::Constant,
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
        //         StatementKind::Constant { .. } | StatementKind::Undefined => {
        //             Some(ValueClass::Constant)
        //         }

        //         StatementKind::ReadRegister { .. }
        //         | StatementKind::ReadMemory { .. }
        //         | StatementKind::ReadPc => Some(ValueClass::Dynamic),
        //         // complicated! todo: be more precise here
        //         StatementKind::ReadVariable { .. } | StatementKind::WriteVariable { .. } => {
        //             Some(ValueClass::Dynamic)
        //         }

        //         StatementKind::WriteRegister { .. }
        //         | StatementKind::WriteMemory { .. }
        //         | StatementKind::Jump { .. }
        //         | StatementKind::Branch { .. }
        //         | StatementKind::Return { .. }
        //         | StatementKind::WritePc { .. } => Some(ValueClass::None),

        //         StatementKind::PhiNode { .. } => todo!(),

        //         // todo: fix panic when this is correctly changed to valueclass::none
        //         StatementKind::Panic(_) => Some(ValueClass::Static),
        //         StatementKind::PrintChar(_) => Some(ValueClass::Static),

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

        // let value_classes = classes.values().cloned().collect::<HashSet<_>>();

        // if value_classes.contains(&ValueClass::Dynamic) {
        //     ValueClass::Dynamic
        // } else if value_classes.contains(&ValueClass::Static) {
        //     ValueClass::Static
        // } else {
        //     ValueClass::Constant
        // }
    }
}
