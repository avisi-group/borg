//! Imported OCaml functions

use crate::type_check::SideEffectInfo;

use {
    crate::{ast::Ast, error::WrapperError, num::BigInt, type_check::Env},
    ocaml::Value,
    std::collections::LinkedList,
};

ocaml::import! {
    // PRIMARY FUNCTIONS
    pub fn util_dedup(l: LinkedList<i32>) -> Result<LinkedList<i32>, WrapperError>;

    // ?loc:Parse_ast.l -> string -> string -> Lexer.comment list * Parse_ast.def list
    pub fn parse_file(contents: String, filename: String) -> Result<(LinkedList<Value>, LinkedList<Value>), WrapperError>;

    // string ->
    // string option ->
    // (Arg.key * Arg.spec * Arg.doc) list ->
    // Parse_ast.def list -> Parse_ast.def list
    pub fn preprocess(sail_dir: String, target: Option<Value>, options: LinkedList<Value>, file_ast: Value) -> Result<LinkedList<Value>, WrapperError>;

    // (string * Parse_ast.def list) list ->
    // (string * Lexer.comment list) list ->
    // Type_check.env ->
    // Type_check.tannot Ast_defs.ast * Type_check.env * Effects.side_effect_info
    pub fn process(defs: LinkedList<(String, LinkedList<Value>)>, comments: LinkedList<(String, LinkedList<Value>)>, type_env: Value) -> Result<(Value, Value, SideEffectInfo), WrapperError>;

    // val descatter :
    //     Effects.side_effect_info ->
    //     Type_check.Env.t ->
    //     Type_check.tannot Ast_defs.ast ->
    //     Type_check.tannot Ast_defs.ast * Type_check.Env.t
    pub fn descatter(effect_info: SideEffectInfo, env: Value, ast: Value) -> Result<(Ast, Env), WrapperError>;

    pub fn type_check_initial_env() -> Result<Value, WrapperError>;

    // CLI OPTIONS

    pub fn set_non_lexical_flow(b: bool) -> Result<(), WrapperError>;

    pub fn set_no_lexp_bounds_check(b: bool) -> Result<(), WrapperError>;

    // UTILITY

    pub fn bindings_to_list(input: Value) -> Result<Value, WrapperError>;

    pub fn list_to_bindings(input: Value) -> Result<Value, WrapperError>;

    pub fn effectset_elements(input: Value) -> Result<Value, WrapperError>;

    pub fn effectset_of_list(input: Value) -> Result<Value, WrapperError>;

    pub fn bigint_to_string(input: Value) -> Result<String, WrapperError>;

    pub fn string_to_bigint(input: String) -> Result<Value, WrapperError>;

    pub fn add_num(a: BigInt, b: BigInt) -> Result<BigInt, WrapperError>;
}
