use {
    crate::{
        ast::{
            generated::{Effect, Identifier, Typ, TypQuant, TypeUnion},
            Mut,
        },
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
    _defined_val_specs: Value,
    locals: Value,
    _top_letbinds: Value,
    union_ids: Value,
    registers: Value,
    variants: Value,
    scattered_variant_envs: Value,
    mappings: Value,
    _typ_vars: Value,
    _shadow_vars: Value,
    _typ_synonyms: Value,
    _overloads: Value,
    _enums: Value,
    _records: Value,
    _accessors: Value,
    _externs: Value,
    casts: LinkedList<Identifier>,
    allow_casts: bool,
    allow_bindings: bool,
    _constraints: LinkedList<Value>,
    _default_order: Option<Value>,
    _ret_typ: Option<Value>,
    _poly_undefineds: bool,
    _prove: Option<Value>,
    allow_unknowns: bool,
    _bitfields: Value,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub top_val_specs: LinkedList<(Identifier, (TypQuant, Typ))>,
    // defined_val_specs: Value,
    pub locals: LinkedList<(Identifier, (Mut, Typ))>,
    // top_letbinds: Value,
    pub union_ids: LinkedList<(Identifier, (TypQuant, Typ))>,
    pub registers: LinkedList<(Identifier, (Effect, Effect, Typ))>,
    pub variants: LinkedList<(Identifier, (TypQuant, LinkedList<TypeUnion>))>,
    pub scattered_variant_envs: Vec<(Identifier, Env)>,
    pub mappings: LinkedList<(Identifier, (TypQuant, Typ, Typ))>,
    // typ_vars: Value,
    // shadow_vars: Value,
    // typ_synonyms: Value,
    // overloads: Value,
    // enums: Value,
    // records: Value,
    // accessors: Value,
    // externs: Value,
    pub casts: LinkedList<Identifier>,
    pub allow_casts: bool,
    pub allow_bindings: bool,
    // constraints: LinkedList<Value>,
    // default_order: Option<Value>,
    // ret_typ: Option<Value>,
    // poly_undefineds: bool,
    // prove: Option<Value>,
    pub allow_unknowns: bool,
    // bitfields: Map<Map<(u64, u64)>>,
}

impl Env {
    pub fn from_value(rt: &mut Runtime, env: Value) -> Result<Self, Error> {
        let raw_env = RawEnv::from_value(env);

        let mut scattered_variant_envs = vec![];
        for (id, value) in
            unsafe { internal_bindings_to_list(rt, raw_env.scattered_variant_envs)?? }
                .into::<LinkedList<(Identifier, Value)>>()
                .into_iter()
        {
            scattered_variant_envs.push((id, Env::from_value(rt, value)?));
        }

        Ok(Self {
            top_val_specs: unsafe { internal_bindings_to_list(rt, raw_env.top_val_specs)?? }.into(),
            locals: unsafe { internal_bindings_to_list(rt, raw_env.locals)?? }.into(),
            union_ids: unsafe { internal_bindings_to_list(rt, raw_env.union_ids)?? }.into(),
            registers: unsafe { internal_bindings_to_list(rt, raw_env.registers)?? }.into(),
            variants: unsafe { internal_bindings_to_list(rt, raw_env.variants)?? }.into(),
            scattered_variant_envs,
            mappings: unsafe { internal_bindings_to_list(rt, raw_env.mappings)?? }.into(),
            casts: raw_env.casts,
            allow_casts: raw_env.allow_casts,
            allow_bindings: raw_env.allow_bindings,
            allow_unknowns: raw_env.allow_unknowns,
        })
    }
}
