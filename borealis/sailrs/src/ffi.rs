#![allow(missing_docs)]

//! Imported OCaml functions

use {
    crate::{error::WrapperError, num::BigInt, types::ListVec},
    ocaml::Value,
};

ocaml::import! {
    pub fn run_sail(filepaths: ListVec<String>) -> Result<Value, WrapperError>;

    // Utility
    pub fn util_dedup(l: ListVec<i32>) -> Result<ListVec<i32>, WrapperError>;
    pub fn bindings_to_list(input: Value) -> Result<Value, WrapperError>;
    pub fn list_to_bindings(input: Value) -> Result<Value, WrapperError>;
    pub fn effectset_elements(input: Value) -> Result<Value, WrapperError>;
    pub fn effectset_of_list(input: Value) -> Result<Value, WrapperError>;
    pub fn bigint_to_string(input: Value) -> Result<String, WrapperError>;
    pub fn string_to_bigint(input: String) -> Result<Value, WrapperError>;
    pub fn add_num(a: BigInt, b: BigInt) -> Result<BigInt, WrapperError>;
}
