//! Sail frontend for GenSim

use {
    crate::brig::sail_to_brig,
    common::{
        bytes,
        intern::{init_interner, interner},
        HashMap,
    },
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::trace,
    rkyv::Deserialize,
    sailrs::{jib_ast::Definition, types::ListVec},
    std::{fs::File, io::Write, path::Path},
};

pub mod boom;
pub mod brig;
pub mod rudder;

pub fn run<I: AsRef<Path>, O: AsRef<Path>>(input: I, output: O, standalone: bool) {
    let jib = load_model(input);

    let tokens = sail_to_brig(jib.into_iter(), standalone);

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

    trace!("deserializing");

    let (jib, strs): (ListVec<Definition>, _) =
        unsafe { rkyv::archived_root::<(ListVec<Definition>, HashMap<String, u32>)>(&mmap) }
            .deserialize(&mut rkyv::Infallible)
            .unwrap();

    trace!("initializing interner");

    init_interner(&strs);

    trace!("JIB size: {:.}", bytes(jib.deep_size_of()));
    trace!(
        "INTERNER size: {:.}, {} strings",
        bytes(interner().current_memory_usage()),
        interner().len()
    );

    jib
}
