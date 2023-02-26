//! Type checking

use {
    crate::{
        ast::{Identifier, Mut, Typ, TypQuant, TypeUnion},
        ffi::{bindings_to_list, effectset_elements, effectset_of_list, list_to_bindings},
    },
    deepsize::DeepSizeOf,
    ocaml::{FromValue, Runtime, ToValue, Value},
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
            bindings_to_list(rt, raw_env.scattered_variant_envs)
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
                bindings_to_list(rt, raw_env.top_val_specs)
                    .unwrap()
                    .unwrap()
            }
            .into(),
            locals: unsafe { bindings_to_list(rt, raw_env.locals).unwrap().unwrap() }.into(),
            union_ids: unsafe { bindings_to_list(rt, raw_env.union_ids).unwrap().unwrap() }.into(),
            variants: unsafe { bindings_to_list(rt, raw_env.variants).unwrap().unwrap() }.into(),
            scattered_variant_envs,
            mappings: unsafe { bindings_to_list(rt, raw_env.mappings).unwrap().unwrap() }.into(),
            casts: raw_env.casts,
            allow_casts: raw_env.allow_casts,
            allow_bindings: raw_env.allow_bindings,
            allow_unknowns: raw_env.allow_unknowns,
        }
    }
}

/// Side effect
#[derive(Debug, Clone, FromValue, ToValue, Serialize, Deserialize, DeepSizeOf)]
pub enum SideEffect {
    /// Throws exception
    Throw,
    /// Exit statement
    Exit,
    /// Incomplete pattern match
    IncompleteMatch,
    /// Register access
    Register,
    /// Calls external function not marked pure
    External,
    /// Contains undefined literal
    Undefined,
    /// Scattered function
    Scattered,
    /// Not executable
    NonExec,
    /// Outcome
    Outcome(Identifier),
}

/// Side effect info
#[derive(Debug, Clone)]
pub struct SideEffectInfo {
    /// Function side effects
    pub functions: LinkedList<(Identifier, LinkedList<SideEffect>)>,
    /// Letbind side effects
    pub letbinds: LinkedList<(Identifier, LinkedList<SideEffect>)>,
    /// Mapping side effects
    pub mappings: LinkedList<(Identifier, LinkedList<SideEffect>)>,
}

unsafe impl FromValue for SideEffectInfo {
    fn from_value(v: Value) -> Self {
        let rt = unsafe { ocaml::Runtime::recover_handle() };

        let raw = <(Value, Value, Value)>::from_value(v);

        Self {
            functions: effectset_bindings_to_list(rt, raw.0),
            letbinds: effectset_bindings_to_list(rt, raw.1),
            mappings: effectset_bindings_to_list(rt, raw.2),
        }
    }
}

fn effectset_bindings_to_list(
    rt: &Runtime,
    value: Value,
) -> LinkedList<(Identifier, LinkedList<SideEffect>)> {
    <LinkedList<(Identifier, Value)>>::from_value(
        unsafe { bindings_to_list(rt, value) }.unwrap().unwrap(),
    )
    .into_iter()
    .map(|(id, value)| {
        (
            id,
            unsafe { effectset_elements(rt, value) }
                .unwrap()
                .unwrap()
                .into(),
        )
    })
    .collect()
}

unsafe impl ToValue for SideEffectInfo {
    fn to_value(&self, rt: &Runtime) -> Value {
        let functions = list_to_effectset_bindings(rt, &self.functions);
        let letbinds = list_to_effectset_bindings(rt, &self.letbinds);
        let mappings = list_to_effectset_bindings(rt, &self.mappings);

        (functions, letbinds, mappings).to_value(rt)
    }
}

fn list_to_effectset_bindings(
    rt: &Runtime,
    list: &LinkedList<(Identifier, LinkedList<SideEffect>)>,
) -> Value {
    let list = list
        .iter()
        .map(|(id, effects)| {
            (
                id,
                unsafe { effectset_of_list(rt, effects.to_value(rt)) }
                    .unwrap()
                    .unwrap(),
            )
        })
        .collect::<LinkedList<_>>()
        .to_value(rt);

    unsafe { list_to_bindings(rt, list) }.unwrap().unwrap()
}
