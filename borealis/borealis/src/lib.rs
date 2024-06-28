//! Sail frontend for GenSim

use {
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
    std::{fs::File, path::Path},
};

pub mod boom;
pub mod codegen;
pub mod rudder;

/// Deserializes an AST from an archive.
///
/// Internally, deserialization is performed on a new thread with a sufficient
/// stack size to perform the deserialization.
pub fn load_model(path: &Path) -> ListVec<Definition> {
    let file = File::open(path).map_err(PathCtx::f(path)).unwrap();
    let mmap = unsafe { memmap2::Mmap::map(&file) }.unwrap();

    trace!("deserializing");

    let (jib, strs): (ListVec<Definition>, _) =
        unsafe { rkyv::archived_root::<(ListVec<Definition>, HashMap<String, u32>)>(&mmap) }
            .deserialize(&mut rkyv::Infallible)
            .unwrap();

    trace!("initializing interner");

    init_interner(&strs);

    trace!("JIB size: {:.2}", bytes(jib.deep_size_of()));
    trace!(
        "INTERNER size: {:.2}, {} strings",
        bytes(interner().current_memory_usage()),
        interner().len()
    );

    jib
}
