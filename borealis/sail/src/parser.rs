//! Sail source file parsing

use crate::{ast::Ast, error::Error, type_check::Env, types::OCamlString, RT};

/// Parses supplied Sail files and returns the AST
///
/// From Sail internal docs:
///
/// This function parses all the files passed to Sail, and then concatenates
/// their ASTs. The pre-processor is then run, which evaluates `$directive`
/// statements in Sail, such as
///
/// ```sail
/// $include <prelude.sail>
/// ```
///
/// Unlike the C pre-processor the Sail pre-processor operates over actual
/// Sail ASTs rather than strings. This can recursively include other
/// files into the AST, as well as add/remove parts of the AST with
/// `$ifdef` etc. Directives that are used are preserved in the AST, so
/// they also function as a useful way to pass auxiliary information to
/// the various Sail backends.
///
/// The initial check mentioned above is then run to desugar the AST, and
/// then the type-checker is run which produces a fully type-checked
/// AST. Type annotations are attached to every node (for which an
/// annotation makes sense) using the aux constructors.
///
/// After type-checking the Sail scattered definitions are de-scattered
/// into single functions.
pub fn load_files(files: Vec<String>) -> Result<(OCamlString, Ast, Env), Error> {
    Ok(RT.lock().load_files(files)?)
}

#[cfg(test)]
mod tests {
    use {
        crate::parser::load_files,
        std::{env, path::PathBuf},
    };

    #[test]
    fn load_files_empty() {
        insta::assert_json_snapshot!(load_files(vec![]).unwrap());
    }

    #[test]
    fn load_files_prelude() {
        let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("examples/prelude.sail")
            .to_string_lossy()
            .to_string();

        insta::assert_json_snapshot!(load_files(vec![path]).unwrap());
    }
}
