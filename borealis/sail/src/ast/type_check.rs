use {
    crate::{
        ast::parse::{Identifier, TypQuant},
        error::Error,
        runtime::internal_bindings_to_list,
    },
    ocaml::{FromValue, Runtime, Value},
    std::{collections::LinkedList, fmt::Debug},
};

/// Env type as can be automatically derived, requires further parsing
#[derive(Debug, Clone, FromValue)]
struct RawEnv {
    top_val_specs: Value,
    defined_val_specs: Value,
    locals: Value,
    top_letbinds: Value,
    union_ids: Value,
    registers: Value,
    variants: Value,
    scattered_variant_envs: Value,
    mappings: Value,
    typ_vars: Value,
    shadow_vars: Value,
    typ_synonyms: Value,
    overloads: Value,
    enums: Value,
    records: Value,
    accessors: Value,
    externs: Value,
    casts: LinkedList<Identifier>,
    allow_casts: bool,
    allow_bindings: bool,
    constraints: LinkedList<Value>,
    default_order: Option<Value>,
    ret_typ: Option<Value>,
    poly_undefineds: bool,
    prove: Option<Value>,
    allow_unknowns: bool,
    bitfields: Value,
}

// #[derive(Debug, Clone)]
// struct Map<T: FromValue + Debug + Clone>(Vec<T>);

// unsafe impl<T: FromValue + Debug + Clone> FromValue for Map<T> {
//     fn from_value(v: Value) -> Self {
//         todo!()
//     }
// }

#[derive(Debug, Clone)]
pub struct Env {
    top_val_specs: LinkedList<(Identifier, (Value, Value))>,
    // defined_val_specs: Value,
    // locals: Value,
    // top_letbinds: Value,
    // union_ids: Value,
    // registers: Value,
    // variants: Value,
    // scattered_variant_envs: Value,
    // mappings: Value,
    // typ_vars: Value,
    // shadow_vars: Value,
    // typ_synonyms: Value,
    // overloads: Value,
    // enums: Value,
    // records: Value,
    // accessors: Value,
    // externs: Value,
    pub casts: LinkedList<Identifier>,
    // allow_casts: bool,
    // allow_bindings: bool,
    // constraints: LinkedList<Value>,
    // default_order: Option<Value>,
    // ret_typ: Option<Value>,
    // poly_undefineds: bool,
    // prove: Option<Value>,
    // allow_unknowns: bool,
    // bitfields: Map<Map<(u64, u64)>>,
}

unsafe impl Send for Env {}
unsafe impl Sync for Env {}

impl Env {
    pub fn from_value(rt: &mut Runtime, env: Value) -> Result<Self, Error> {
        let raw_env = RawEnv::from_value(env);

        dbg!(&raw_env);

        Ok(Self {
            top_val_specs: unsafe { internal_bindings_to_list(rt, raw_env.top_val_specs)?? }.into(),
            casts: raw_env.casts,
        })
    }
}
