use {
    crate::rudder::{
        function_inliner,
        model::{function::Function, types::Type},
        opt, validator,
    },
    sailrs::{intern::InternedString, HashMap},
};

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

    pub fn optimise(&mut self, level: opt::OptLevel) {
        opt::optimise(self, level);
    }

    pub fn validate(&mut self) -> Vec<validator::ValidationMessage> {
        validator::validate(self)
    }

    pub fn function_inline(&mut self, top_level_fns: &[&'static str]) {
        function_inliner::inline(self, top_level_fns);
    }

    pub fn get_functions(&self) -> &HashMap<InternedString, Function> {
        &self.fns
    }

    pub fn get_functions_mut(&mut self) -> &mut HashMap<InternedString, Function> {
        &mut self.fns
    }

    pub fn registers(&self) -> &HashMap<InternedString, RegisterDescriptor> {
        &self.registers
    }
}
