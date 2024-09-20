use {
    crate::rudder::{
        function_inliner,
        model::{function::Function, types::Type},
        opt, validator,
    },
    common::{intern::InternedString, HashMap, HashSet},
};

pub mod block;
pub mod constant_value;
pub mod function;
pub mod statement;
pub mod types;

#[derive(Default)]
pub struct Model {
    pub(crate) fns: HashMap<InternedString, Function>,
    // offset-type pairs, offsets may not be unique? todo: ask tom
    pub(crate) registers: HashMap<InternedString, RegisterDescriptor>,
    pub(crate) structs: HashSet<Type>,
}

#[derive(Clone, Debug)]
pub struct RegisterDescriptor {
    pub typ: Type,
    pub offset: usize,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_function(&mut self, name: InternedString, func: Function) {
        self.fns.insert(name, func);
    }

    pub fn update_names(&mut self) {
        for func in self.fns.values_mut() {
            func.update_indices();
        }
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

    pub fn get_registers(&self) -> HashMap<InternedString, RegisterDescriptor> {
        self.registers.clone()
    }

    pub fn get_structs(&self) -> HashSet<Type> {
        self.structs.clone()
    }
}
