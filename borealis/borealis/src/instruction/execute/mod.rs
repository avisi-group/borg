//! Instruction execute behaviour
//!
//! Compiles the JIB for a

use {
    crate::instruction::execute::pretty_print::print_instructions,
    common::intern::InternedStringKey, sail::jib_ast::Definition, std::collections::LinkedList,
};

pub mod pretty_print;

/// Compiles the JIB AST of a function to an equivalent GenC function
pub fn jib_func_to_genc(
    function_identifier: InternedStringKey,
    jib: &LinkedList<Definition>,
) -> String {
    if function_identifier != "integer_arithmetic_addsub_immediate_decode".into() {
        return "".to_owned();
    }

    let Some(Definition::Fundef(_ident, _, _parameters, instructions)) = jib
        .iter()
        .find(|def| match def {
            Definition::Fundef(ident, _, _, _) => ident.get_string() == function_identifier,
            _ => false,
        }) else {
            panic!("failed to find function definition with identifier {function_identifier} in JIB AST");
        };

    print_instructions(instructions);

    "".to_owned()
}
