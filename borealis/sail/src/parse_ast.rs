//! Sail abstract syntax tree corresponding to data structures in `parse_ast.ml`,
//! which itself is generated from `l2_parse.ott`.

use {
    crate::ast::L,
    common::intern::InternedStringKey,
    ocaml::{FromValue, ToValue, Value},
    std::collections::LinkedList,
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
    Overload(Value, LinkedList<Value>),

    /// Fixity declaration
    Fixity(Value, Value, Value),

    /// Top-level type constraint
    Spec(Value),

    /// Top-level outcome definition
    Outcome(Value, LinkedList<Definition>),

    /// Instantiation
    Instantiation(Value, LinkedList<Value>),

    /// Default type and kind assumptions
    Default(Value),

    /// Scattered definition
    Scattered(Value),

    /// Separate termination measure declaration
    Measure(Value, Value, Value),

    /// Separate termination measure declaration
    LoopMeasures(Value, LinkedList<Value>),

    /// Register declaration
    Register(Value),

    /// Pragma
    Pragma(InternedStringKey, InternedStringKey, L),

    /// Internal mutrec
    Mutual(LinkedList<Value>),
}
