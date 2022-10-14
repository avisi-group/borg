//! Sail source file parsing

use ocaml::interop::{BoxRoot, ToOCaml};

use {
    crate::{error::Error, RT},
    ocaml::{List, Value},
};

ocaml::import! {
    fn internal_type_check_initial_env() -> Value;

    // val load_files : ?check:bool -> (Arg.key * Arg.spec * Arg.doc) list -> Type_check.Env.t -> string list -> (string * Type_check.tannot ast * Type_check.Env.t)
    fn internal_process_file_load_files(check: bool, options: List<Value>, env: Value, files: List<BoxRoot<String>>) -> (String, Value, Value);
}

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
pub fn load_files(files: Vec<String>) -> Result<(), Error> {
    RT.write();

    let env = unsafe { internal_type_check_initial_env(&RT.read())? };

    let files = {
        let mut l = List::empty();
        for file in files {
            let file_rooted: BoxRoot<String> = file.to_boxroot(&mut RT.write());
            l = unsafe { l.add(&RT.read(), &file_rooted) };
        }
        l
    };

    let (output_name, ast, type_envs) =
        unsafe { internal_process_file_load_files(&RT.read(), false, List::empty(), env, files)? };

    dbg!((output_name, ast, type_envs));

    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        crate::parser::load_files,
        std::{env, path::PathBuf},
    };

    #[test]
    fn load_files_empty() {
        load_files(vec![]).unwrap();
    }

    #[test]
    fn load_files_prelude() {
        let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("examples/prelude.sail")
            .to_string_lossy()
            .to_string();

        load_files(vec![path]).unwrap();
    }
}
