//! Instruction execute behaviour
//!
//! Compiles the JIB for a

use {common::intern::InternedString, sail::jib_ast::Definition, std::collections::LinkedList};

pub mod pretty_print;

/// Compiles the JIB AST of a function to an equivalent GenC function
pub fn jib_func_to_genc(
    _function_identifier: InternedString,
    _jib: &LinkedList<Definition>,
) -> String {
    "".to_owned()
}
