//! Sail frontend for GenSim

use {
    crate::rust::sail_to_brig,
    common::intern::INTERNER,
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::trace,
    rkyv::Deserialize,
    sailrs::{jib_ast::Definition, runtime::DEFAULT_RUNTIME_THREAD_STACK_SIZE, types::ListVec},
    std::{fs::File, io::Write, path::Path, thread},
};

pub mod boom;
pub mod passes;
pub mod rudder;
pub mod rust;

pub fn run<I: AsRef<Path>, O: AsRef<Path>>(input: I, output: O, standalone: bool) {
    let jib = load_model(input);

    let tokens = sail_to_brig(jib.as_ref(), standalone);

    let syntax_tree = syn::parse_file(&tokens.to_string()).unwrap();

    let formatted = prettyplease::unparse(&syntax_tree);

    File::create(output)
        .unwrap()
        .write_all(formatted.as_bytes())
        .unwrap();
}

/// Deserializes an AST from an archive.
///
/// Internally, deserialization is performed on a new thread with a sufficient
/// stack size to perform the deserialization.
fn load_model<P: AsRef<Path>>(path: P) -> ListVec<Definition> {
    let file = File::open(&path).map_err(PathCtx::f(&path)).unwrap();
    let mmap = unsafe { memmap2::Mmap::map(&file) }.unwrap();

    let thread = thread::Builder::new().stack_size(DEFAULT_RUNTIME_THREAD_STACK_SIZE);

    let handle = thread
        .spawn(move || {
            trace!("deserializing");
            let jib = unsafe { rkyv::archived_root::<ListVec<Definition>>(&mmap) }
                .deserialize(&mut rkyv::Infallible)
                .unwrap();
            trace!("done");
            jib
        })
        .map_err(PathCtx::f(&path))
        .unwrap();

    let jib: ListVec<Definition> = handle
        .join()
        .expect("Failed to join on deserializing thread");

    trace!("JIB size: {} bytes", jib.deep_size_of());
    trace!(
        "INTERNER size: {} bytes, {} strings",
        INTERNER.current_memory_usage(),
        INTERNER.len()
    );

    jib
}
