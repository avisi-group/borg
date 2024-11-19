use crate::{
    intern::InternedString,
    rudder::{
        function::Function,
        types::{maybe_type_to_string, Type},
    },
    HashMap,
};
use alloc::{borrow::ToOwned, format, string::ToString};
use core::fmt::{self, Display, Formatter};
use itertools::Itertools;

pub mod block;
pub mod constant_value;
pub mod function;
pub mod statement;
pub mod types;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Model {
    fns: HashMap<InternedString, Function>,
    // offset-type pairs, offsets may not be unique? todo: ask tom
    registers: HashMap<InternedString, RegisterDescriptor>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RegisterDescriptor {
    pub typ: Type,
    pub offset: usize,
}

impl Model {
    pub fn new(
        fns: HashMap<InternedString, Function>,
        registers: HashMap<InternedString, RegisterDescriptor>,
    ) -> Self {
        Self { fns, registers }
    }

    pub fn add_function(&mut self, name: InternedString, func: Function) {
        self.fns.insert(name, func);
    }

    pub fn functions(&self) -> &HashMap<InternedString, Function> {
        &self.fns
    }

    pub fn functions_mut(&mut self) -> &mut HashMap<InternedString, Function> {
        &mut self.fns
    }

    pub fn registers(&self) -> &HashMap<InternedString, RegisterDescriptor> {
        &self.registers
    }

    pub fn register_file_size(&self) -> usize {
        self.registers
            .values()
            .map(|d| d.offset + d.typ.width_bytes())
            .max()
            .unwrap()
    }

    pub fn reg_offset(&self, name: &'static str) -> usize {
        self.registers
            .get(&InternedString::from_static(name))
            .unwrap_or_else(|| panic!("no register found with name {name:?}"))
            .offset
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "rudder context:")?;
        writeln!(f)?;

        for (name, reg) in self.registers() {
            writeln!(f, "register {name}: {reg:?}")?;
        }

        writeln!(f)?;
        for (name, func) in self.functions() {
            let parameters = func
                .parameters()
                .into_iter()
                .map(|sym| format!("{}: {}", sym.name(), sym.typ()))
                .join(", ");
            writeln!(
                f,
                "function {}({}) -> {} :",
                name,
                parameters,
                maybe_type_to_string(func.return_type())
            )?;

            write!(f, "{}", func)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.block_iter().try_for_each(|block| {
            writeln!(f, "  block {:#x}:", block.index())?;
            write!(f, "{}", block.get(self.arena()))
        })
    }
}
