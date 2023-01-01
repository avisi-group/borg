//! Imported OCaml functions

use {
    crate::error::WrapperError,
    ocaml::{List, Value},
    std::collections::LinkedList,
};

ocaml::import! {
    // PRIMARY FUNCTIONS
    pub fn util_dedup(l: List<Value>) -> Result<List<i32>, WrapperError>;

    pub fn parse_file(contents: String, filename: String) -> Result<(Value, Value), WrapperError>;

    pub fn preprocess(sail_dir: String, target: Option<Value>, options: List<Value>, file_ast: Value) -> Result<List<Value>, WrapperError>;

    pub fn process(defs: LinkedList<(String, List<Value>)>, comments: LinkedList<(String, Value)>, type_env: Value) -> Result<(Value, Value, Value), WrapperError>;

    // val descatter :
    //     Effects.side_effect_info ->
    //     Type_check.Env.t ->
    //     Type_check.tannot Ast_defs.ast ->
    //     Type_check.tannot Ast_defs.ast * Type_check.Env.t
    pub fn descatter(effect_info: Value, env: Value, ast: Value) -> Result<(Value, Value), WrapperError>;

    pub fn type_check_initial_env() -> Result<Value, WrapperError>;

    // CLI OPTIONS

    pub fn set_non_lexical_flow(b: bool) -> Result<(), WrapperError>;

    pub fn set_no_lexp_bounds_check(b: bool) -> Result<(), WrapperError>;

    // UTILITY

    pub fn bindings_to_list(input: Value) -> Result<Value, WrapperError>;

    pub fn bigint_to_string(input: Value) -> Result<String, WrapperError>;

    pub fn add_num(a: String, b: String) -> Result<String, WrapperError>;

}
