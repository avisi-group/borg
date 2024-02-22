#![warn(missing_docs)]

//! Rust interface to `Sail` compiler library

use {
    crate::{
        error::Error,
        ffi::{generate_jib, parse_file, preprocess, run_sail},
        json::ModelConfig,
        parse_ast::Definition,
        runtime::RT,
        sail_ast::Ast,
    },
    common::intern::InternedString,
    errctx::PathCtx,
    include_dir::{include_dir, Dir},
    log::trace,
    ocaml::{FromValue, Runtime, Value},
    std::{collections::LinkedList, fs::read_to_string, path::Path},
};

pub mod error;
pub mod ffi;
pub mod jib_ast;
pub mod json;
pub mod num;
pub mod parse_ast;
pub mod runtime;
pub mod sail_ast;
pub mod type_check;
pub mod types;

/// Sail standard library files
static SAIL_LIB: Dir = include_dir!("$CARGO_MANIFEST_DIR/lib");

/// Loads Sail files from `sail.json` model configuration.
///
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
pub fn load_from_config<P: AsRef<Path>>(
    config_path: P,
) -> Result<(Ast, LinkedList<jib_ast::Definition>), Error> {
    let ModelConfig { files, .. } = ModelConfig::load(config_path.as_ref())?;

    RT.lock().execute(move |rt| {
        trace!("Preprocessing");
        let mut defs = LinkedList::new();
        let mut comments = LinkedList::new();

        for path in files {
            trace!("Preprocessing {:?}", path);
            let contents = read_to_string(&path).map_err(PathCtx::f(&path))?;

            // file path used for AST location annotation
            let path = path.as_os_str().to_string_lossy().to_string();

            let (file_comments, file_defs) = preprocess_file(rt, contents, path.clone())?;

            defs.push_back((path.clone(), file_defs));
            comments.push_back((path.clone(), file_comments));
        }

        trace!("Compiling Sail");
        let (ast, env, effect_info) = unsafe { run_sail(rt, defs, comments) }??;

        trace!("Generating JIB IR");
        let jib = unsafe { generate_jib(rt, ast.clone(), env, effect_info) }??;

        let ast = Ast::from_value(ast);
        let jib = LinkedList::<jib_ast::Definition>::from_value(jib);

        Ok((ast, jib))
    })?
}

fn preprocess_file(
    rt: &Runtime,
    contents: String,
    path: String,
) -> Result<(LinkedList<Value>, LinkedList<Definition>), Error> {
    trace!("Parsing {:?}", path);

    let (comments, file_ast) = unsafe { parse_file(rt, contents, path.clone()) }??;

    trace!("Importing includes for {:?}", path);

    let file_ast = resolve_includes(rt, file_ast)?;

    trace!("Preprocessing {:?}", path);

    let defs = unsafe { preprocess(rt, "".to_owned(), None, LinkedList::new(), file_ast) }??;

    Ok((comments, defs))
}

/// Finds `$include <filename.sail>` pragmas and replaces with the parsed AST of
/// the referenced Sail standard library file
fn resolve_includes(
    rt: &Runtime,
    file_ast: LinkedList<Definition>,
) -> Result<LinkedList<Definition>, Error> {
    let mut out_defs = LinkedList::new();

    for def in file_ast {
        let Definition::Pragma(key, value, l) = &def else {
            out_defs.push_back(def);
            continue;
        };

        if *key != InternedString::from_static("include") {
            out_defs.push_back(def);
            continue;
        }

        let include = value.to_string();

        if !(include.starts_with('<') && include.ends_with(".sail>")) {
            out_defs.push_back(def);
            continue;
        }

        let filename = include
            .strip_prefix('<')
            .unwrap()
            .strip_suffix('>')
            .unwrap();

        let Some(file) = SAIL_LIB.get_file(filename) else {
            return Err(Error::MissingIncludeFile(filename.to_owned(), l.clone()));
        };

        let contents = file.contents_utf8().unwrap();

        trace!("Found {:?} in SAIL_LIB, recursing", filename);
        let (_, mut defs) = preprocess_file(rt, contents.to_owned(), filename.to_owned())?;

        if !defs.is_empty() {
            out_defs.push_back(Definition::Pragma(
                "include_start".into(),
                filename.into(),
                l.clone(),
            ));
            out_defs.append(&mut defs);
            out_defs.push_back(Definition::Pragma(
                "include_end".into(),
                filename.into(),
                l.clone(),
            ));
        }
    }

    Ok(out_defs)
}

#[cfg(test)]
mod tests {
    use {crate::load_from_config, once_cell::sync::Lazy};

    const FILTERS: Lazy<Vec<(&'static str, &'static str)>> = Lazy::new(|| {
        vec![
            (r#""[0-9a-zA-Z\.\-/+]+/(?P<n>.*\.sail)""#, r#""$n""#),
            (
                r#""kind_identifier": \[[\s,0-9]*\]"#,
                r#""kind_identifier": []"#,
            ),
        ]
    });

    #[test]
    fn load_files_empty() {
        let (ast, _) =
            load_from_config(concat!(env!("CARGO_MANIFEST_DIR"), "/../data/empty.json")).unwrap();

        insta::with_settings!({filters => FILTERS.clone()}, {
            insta::assert_json_snapshot!((ast));
        });
    }
}
