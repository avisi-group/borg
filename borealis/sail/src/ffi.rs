#![allow(missing_docs)]

//! Imported OCaml functions

use {
    crate::{error::WrapperError, num::BigInt, parse_ast::Definition},
    ocaml::Value,
    std::collections::LinkedList,
};

ocaml::import! {
    // ?loc:Parse_ast.l -> string -> string -> Lexer.comment list * Parse_ast.def list
    pub fn parse_file(contents: String, filename: String) -> Result<(LinkedList<Value>, LinkedList<Definition>), WrapperError>;

    // string ->
    // string option ->
    // (Arg.key * Arg.spec * Arg.doc) list ->
    // Parse_ast.def list -> Parse_ast.def list
    pub fn preprocess(sail_dir: String, target: Option<Value>, options: LinkedList<Value>, file_ast: LinkedList<Definition>) -> Result<LinkedList<Definition>, WrapperError>;

    pub fn run_sail( definitions: LinkedList<(String, LinkedList<Definition>)>, comments: LinkedList<(String, LinkedList<Value>)>) -> Result<(Value, Value, Value), WrapperError>;

    pub fn generate_jib(ast: Value, effect_info: Value, env: Value) -> Result<Value, WrapperError>;

    // Utility
    pub fn util_dedup(l: LinkedList<i32>) -> Result<LinkedList<i32>, WrapperError>;
    pub fn bindings_to_list(input: Value) -> Result<Value, WrapperError>;
    pub fn list_to_bindings(input: Value) -> Result<Value, WrapperError>;
    pub fn effectset_elements(input: Value) -> Result<Value, WrapperError>;
    pub fn effectset_of_list(input: Value) -> Result<Value, WrapperError>;
    pub fn bigint_to_string(input: Value) -> Result<String, WrapperError>;
    pub fn string_to_bigint(input: String) -> Result<Value, WrapperError>;
    pub fn add_num(a: BigInt, b: BigInt) -> Result<BigInt, WrapperError>;
}
