use {
    crate::{
        intern::InternedString,
        rudder::{
            function::Function,
            types::{maybe_type_to_string, Type},
        },
        HashMap,
    },
    alloc::{collections::btree_map::BTreeMap, format},
    core::fmt::{self, Display, Formatter},
    itertools::Itertools,
};

pub mod block;
pub mod constant_value;
pub mod function;
pub mod statement;
pub mod types;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Model {
    functions: HashMap<InternedString, Function>,
    registers: HashMap<InternedString, RegisterDescriptor>,
    // todo: wastes memory when serialized, don't serialize and regenerate when deserializing?
    registers_by_offset: BTreeMap<u64, InternedString>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RegisterDescriptor {
    pub typ: Type,
    pub offset: u64,
    /// Registers that change infrequently can be cached during translation so reads of these registers are emitted as constant values rather than register reads
    pub cacheable: bool,
}

impl Model {
    pub fn new(
        fns: HashMap<InternedString, Function>,
        registers: HashMap<InternedString, RegisterDescriptor>,
    ) -> Self {
        let registers_by_offset = registers
            .iter()
            .map(|(name, RegisterDescriptor { offset, .. })| (*offset, *name))
            .collect();

        Self {
            functions: fns,
            registers,
            registers_by_offset,
        }
    }

    pub fn add_function(&mut self, name: InternedString, func: Function) {
        self.functions.insert(name, func);
    }

    pub fn functions(&self) -> &HashMap<InternedString, Function> {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut HashMap<InternedString, Function> {
        &mut self.functions
    }

    pub fn registers(&self) -> &HashMap<InternedString, RegisterDescriptor> {
        &self.registers
    }

    pub fn register_file_size(&self) -> u64 {
        self.registers
            .values()
            .map(|d| d.offset + u64::from(d.typ.width_bytes()))
            .max()
            .unwrap()
    }

    pub fn reg_offset(&self, name: &'static str) -> u64 {
        self.registers
            .get(&InternedString::from_static(name))
            .unwrap_or_else(|| panic!("no register found with name {name:?}"))
            .offset
    }

    pub fn get_register_by_offet(&self, offset: u64) -> Option<InternedString> {
        self.registers_by_offset
            .range(..=offset)
            .map(|(_, name)| *name)
            .next_back()
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "rudder context:")?;
        writeln!(f)?;

        for (name, reg) in self
            .registers()
            .into_iter()
            .sorted_by(|a, b| a.0.as_ref().cmp(b.0.as_ref()))
        {
            writeln!(f, "register {name}: {reg:?}")?;
        }

        writeln!(f)?;
        for (name, func) in self
            .functions()
            .into_iter()
            .sorted_by(|a, b| a.0.as_ref().cmp(b.0.as_ref()))
        {
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
