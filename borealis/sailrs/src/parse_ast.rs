//! Sail abstract syntax tree corresponding to data structures in
//! `parse_ast.ml`, which itself is generated from `l2_parse.ott`.

use {
    crate::{sail_ast::Location, types::ListVec},
    common::intern::InternedString,
    ocaml::{FromValue, ToValue, Value},
    strum::IntoStaticStr,
};

/// Top-level Sail2 definition
#[derive(Debug, Clone, PartialEq, FromValue, ToValue, IntoStaticStr)]
pub enum Definition {
    /// Type definition
    Type(Value),

    /// Function definition
    Function(Value),

    /// Mapping definition
    Mapping(Value),

    /// Impl definition
    Impl(Value),

    /// Value definition
    Value(Value),

    /// Operator overload specifications
    Overload(Value, ListVec<Value>),

    /// Fixity declaration
    Fixity(Value, Value, Value),

    /// Top-level type constraint
    Spec(Value),

    /// Top-level outcome definition
    Outcome(Value, ListVec<Definition>),

    /// Instantiation
    Instantiation(Value, ListVec<Value>),

    /// Default type and kind assumptions
    Default(Value),

    /// Scattered definition
    Scattered(Value),

    /// Separate termination measure declaration
    Measure(Value, Value, Value),

    /// Separate termination measure declaration
    LoopMeasures(Value, ListVec<Value>),

    /// Register declaration
    Register(Value),

    /// Pragma
    Pragma(InternedString, InternedString, Location),

    /// Internal mutrec
    Mutual(ListVec<Value>),
}
