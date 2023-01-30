use {
    color_eyre::Result,
    decoder_harness::ffi::{new_decoder, opcode},
    errctx::PathCtx,
    memmap2::Mmap,
    std::fs::File,
};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/input.bin");

fn main() -> Result<()> {
    color_eyre::install()?;

    let file = File::open(INPUT_PATH).map_err(PathCtx::f(INPUT_PATH))?;
    let mmap = unsafe { Mmap::map(&file) }?;

    let mut decoder = new_decoder();

    for chunk in mmap.chunks_exact(4) {
        unsafe { decoder.pin_mut().decode(0, 0, &chunk[0]) };
        println!(
            "{:08X}: {:#?}",
            unsafe { std::mem::transmute::<_, &u32>(&chunk[0]) },
            opcode(&decoder)
        );
    }

    Ok(())
}
