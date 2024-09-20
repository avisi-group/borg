use {
    crate::rudder::{
        analysis::cfg::ControlFlowGraphAnalysis, Block, ConstantValue, Function, Model,
        PrimitiveTypeClass, Symbol, Type,
    },
    itertools::Itertools,
    std::fmt::{Display, Formatter, Result},
};

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Type::Primitive(p) => match &p.tc {
                PrimitiveTypeClass::Void => write!(f, "void"),
                PrimitiveTypeClass::Unit => write!(f, "()"),
                PrimitiveTypeClass::UnsignedInteger => write!(f, "u{}", self.width_bits()),
                PrimitiveTypeClass::SignedInteger => write!(f, "i{}", self.width_bits()),
                PrimitiveTypeClass::FloatingPoint => write!(f, "f{}", self.width_bits()),
            },
            Type::Struct(_) => write!(f, "struct"),
            Type::Union { width } => write!(f, "union({width})"),
            Type::Vector {
                element_count,
                element_type,
            } => write!(f, "[{element_type}; {element_count:?}]"),
            Type::Bits => write!(f, "bv"),
            Type::ArbitraryLengthInteger => write!(f, "i"),
            Type::String => write!(f, "str"),
            Type::Rational => write!(f, "rational"),
            Type::Any => write!(f, "any"),
            Type::Tuple(ts) => {
                write!(f, "(").unwrap();
                for t in ts {
                    write!(f, "{t}, ").unwrap();
                }
                write!(f, ")")
            }
        }
    }
}

impl Display for ConstantValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ConstantValue::UnsignedInteger(v) => write!(f, "{v}u"),
            ConstantValue::SignedInteger(v) => write!(f, "{v}s"),
            ConstantValue::FloatingPoint(v) => write!(f, "{v}f"),
            ConstantValue::Unit => write!(f, "()"),
            ConstantValue::String(str) => write!(f, "{str:?}"),
            ConstantValue::Rational(r) => write!(f, "{r:?}"),
            ConstantValue::Tuple(vs) => {
                write!(f, "(").unwrap();
                vs.iter().for_each(|v| write!(f, "{v},  ").unwrap());
                write!(f, ")")
            }
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for stmt in &self.statements {
            writeln!(
                f,
                "    {}",
                stmt.get(&self.statement_arena)
                    .to_string(&self.statement_arena)
            )?;
        }

        Ok(())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let cfg = ControlFlowGraphAnalysis::new(self);

        self.block_iter().try_for_each(|block| {
            let preds = cfg
                .predecessors_for(block)
                .unwrap()
                .iter()
                .map(|b| b.index())
                .join(", ");

            let succs = cfg
                .successors_for(block)
                .unwrap()
                .iter()
                .map(|b| b.index())
                .join(", ");

            writeln!(
                f,
                "  block{}: preds={{{preds}}}, succs={{{succs}}}",
                block.index()
            )?;
            write!(f, "{}", block.get(self.block_arena()))
        })
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "rudder context:")?;

        for (name, func) in self.fns.iter() {
            writeln!(f, "function {}:", name,)?;

            write!(f, "{}", func)?;
            writeln!(f)?;
        }

        Ok(())
    }
}
