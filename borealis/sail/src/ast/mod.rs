#![allow(missing_docs)]

//! Sail Abstract Syntax Tree

use {
    crate::ffi::{BigNum, OCamlString, Rational},
    ocaml::FromValue,
    std::collections::LinkedList,
};

pub mod generated;
pub mod parse;
pub mod type_check;

#[derive(Debug, Clone, FromValue)]
pub enum Bit {
    B0,
    B1,
}

/// Sail AST Value
///
/// **Not to be confused with `ocaml::Value`**
#[derive(Debug, Clone, FromValue)]
pub enum Value {
    Vector(LinkedList<Value>),
    List(LinkedList<Value>),
    Int(BigNum),
    Real(Rational),
    Bool(bool),
    Bit(Bit),
    Tuple(LinkedList<Value>),
    Unit,
    String(OCamlString),
    Ref(OCamlString),
    Ctor(OCamlString, LinkedList<Value>),
    Record(LinkedList<(String, Value)>),
    AttemptedRead(OCamlString),
}
