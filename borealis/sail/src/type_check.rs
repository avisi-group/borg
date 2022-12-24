//! Type checking

use {
    crate::{
        ast::{Identifier, Mut, Typ, TypQuant, TypeUnion},
        wrapper::internal_bindings_to_list,
    },
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Value},
    serde::{Deserialize, Serialize},
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
    _registers: Value,
    variants: Value,
    scattered_variant_envs: Value,
    mappings: Value,
    _typ_vars: Value,
    _shadow_vars: Value,
    _typ_synonyms: Value,
    _typ_params: Value,
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
    _top_level: Value,
    _outcomes: Value,
    _outcome_typschm: Value,
    _outcome_instantiation: Value,
}

/// Type check environment
#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
pub struct Env {
    /// Top value specifications
    pub top_val_specs: LinkedList<(Identifier, (TypQuant, Typ))>,

    // defined_val_specs: Value,
    /// Local variables
    pub locals: LinkedList<(Identifier, (Mut, Typ))>,

    // top_letbinds: Value,
    /// Union identifiers
    pub union_ids: LinkedList<(Identifier, (TypQuant, Typ))>,

    /// Registers
    //pub registers: LinkedList<(Identifier, (Value, Effect, Typ))>,

    /// Variants
    pub variants: LinkedList<(Identifier, (TypQuant, LinkedList<TypeUnion>))>,

    /// Scattered variant environments
    pub scattered_variant_envs: Vec<(Identifier, Env)>,

    /// Mappings
    pub mappings: LinkedList<(Identifier, (TypQuant, Typ, Typ))>,

    // typ_vars: Value,
    // shadow_vars: Value,
    // typ_synonyms: Value,
    // overloads: Value,
    // enums: Value,
    // records: Value,
    // accessors: Value,
    // externs: Value,
    /// Casts
    pub casts: LinkedList<Identifier>,

    /// Allow casts
    pub allow_casts: bool,

    /// Allow bindings
    pub allow_bindings: bool,

    // constraints: LinkedList<Value>,
    // default_order: Option<Value>,
    // ret_typ: Option<Value>,
    // poly_undefineds: bool,
    // prove: Option<Value>,
    /// Allow unknowns
    pub allow_unknowns: bool,
    // bitfields: Map<Map<(u64, u64)>>,
}

unsafe impl FromValue for Env {
    fn from_value(v: Value) -> Self {
        let raw_env = RawEnv::from_value(v);

        let rt = unsafe { ocaml::Runtime::recover_handle() };

        let mut scattered_variant_envs = vec![];
        for (id, value) in unsafe {
            internal_bindings_to_list(rt, raw_env.scattered_variant_envs)
                .unwrap()
                .unwrap()
        }
        .into::<LinkedList<(Identifier, Value)>>()
        .into_iter()
        {
            scattered_variant_envs.push((id, Env::from_value(value)));
        }

        Self {
            top_val_specs: unsafe {
                internal_bindings_to_list(rt, raw_env.top_val_specs)
                    .unwrap()
                    .unwrap()
            }
            .into(),
            locals: unsafe {
                internal_bindings_to_list(rt, raw_env.locals)
                    .unwrap()
                    .unwrap()
            }
            .into(),
            union_ids: unsafe {
                internal_bindings_to_list(rt, raw_env.union_ids)
                    .unwrap()
                    .unwrap()
            }
            .into(),
            variants: unsafe {
                internal_bindings_to_list(rt, raw_env.variants)
                    .unwrap()
                    .unwrap()
            }
            .into(),
            scattered_variant_envs,
            mappings: unsafe {
                internal_bindings_to_list(rt, raw_env.mappings)
                    .unwrap()
                    .unwrap()
            }
            .into(),
            casts: raw_env.casts,
            allow_casts: raw_env.allow_casts,
            allow_bindings: raw_env.allow_bindings,
            allow_unknowns: raw_env.allow_unknowns,
        }
    }
}
