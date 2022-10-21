use {
    crate::ast::TypQuant,
    ocaml::{FromValue, ToValue, Value},
    std::collections::LinkedList,
};

#[derive(Debug, Clone, FromValue, ToValue)]
pub struct Env {
    pub top_val_specs: Value,
    pub defined_val_specs: Value,
    pub locals: Value,
    pub top_letbinds: Value,
    pub union_ids: Value,
    pub registers: Value,
    pub variants: Value,
    pub scattered_variant_envs: Value,
    pub mappings: Value,
    pub typ_vars: Value,
    pub shadow_vars: Value,
    pub typ_synonyms: Value,
    pub overloads: Value,
    pub enums: Value,
    pub records: Value,
    pub accessors: Value,
    pub externs: Value,
    pub casts: LinkedList<Value>,
    pub allow_casts: bool,
    pub allow_bindings: bool,
    pub constraints: LinkedList<Value>,
    pub default_order: Option<Value>,
    pub ret_typ: Option<Value>,
    pub poly_undefineds: bool,
    pub prove: Option<Value>,
    pub allow_unknowns: bool,
    pub bitfields: Value,
}

unsafe impl Send for Env {}
unsafe impl Sync for Env {}
