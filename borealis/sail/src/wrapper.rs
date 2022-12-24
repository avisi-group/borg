//! Imported OCaml functions

use {
    crate::error::WrapperError,
    ocaml::{interop::BoxRoot, List, Value},
};

ocaml::import! {
    pub fn internal_util_dedup(l: List<Value>) -> Result<List<i32>, WrapperError>;

    pub fn internal_type_check_initial_env() -> Result<Value, WrapperError>;

    // val load_files :
    //     ?target:Target.target ->
    //     string ->
    //     (Stdlib.Arg.key * Stdlib.Arg.spec * Stdlib.Arg.doc) list ->
    //     Type_check.Env.t ->
    //     string list ->
    //     Type_check.tannot Ast_defs.ast * Type_check.Env.t * Effects.side_effect_info
    pub fn internal_load_files(default_sail_dir: BoxRoot<String>, options: List<Value>, type_envs: Value, file_paths: List<BoxRoot<String>>) -> Result<(Value, Value, Value), WrapperError>;

    // val descatter :
    //     Effects.side_effect_info ->
    //     Type_check.Env.t ->
    //     Type_check.tannot Ast_defs.ast ->
    //     Type_check.tannot Ast_defs.ast * Type_check.Env.t
    pub fn internal_descatter(effect_info: Value, env: Value, ast: Value)  -> Result<(Value, Value), WrapperError>;

    pub fn internal_bindings_to_list(input: Value) -> Result<Value, WrapperError>;

    pub fn internal_bigint_to_string(input: Value) -> Result<String, WrapperError>;

    pub fn internal_add_num(a: String, b: String) -> Result<String, WrapperError>;

    pub fn internal_set_non_lexical_flow(b: bool) -> Result<(), WrapperError>;

    pub fn internal_set_no_lexp_bounds_check(b: bool) -> Result<(), WrapperError>;
}
