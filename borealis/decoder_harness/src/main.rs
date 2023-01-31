use decoder_harness::ffi::arm64_decode;

use {
    color_eyre::Result,
    decoder_harness::ffi::{new_decoder, new_disassembler, opcode},
    errctx::PathCtx,
    memmap2::Mmap,
    std::{ffi::CStr, fs::File},
};

const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/asm/aout.bin");

fn main() -> Result<()> {
    color_eyre::install()?;

    let file = File::open(INPUT_PATH)
        .map_err(PathCtx::f(INPUT_PATH))
        .unwrap();
    let mmap = unsafe { Mmap::map(&file) }.unwrap();

    let mut decoder = new_decoder();
    let mut disassembler = new_disassembler();

    for chunk in mmap.chunks_exact(4) {
        unsafe { decoder.pin_mut().decode(0, 0, &chunk[0]) };
        let s = unsafe {
            disassembler.pin_mut().disassemble(
                0,
                (decoder.pin_mut().into_ref().get_ref() as *const arm64_decode) as *const u8,
            )
        };

        println!(
            "{:08X}: {:#4?}: {:?}",
            unsafe { std::mem::transmute::<_, &u32>(&chunk[0]) },
            opcode(&decoder),
            unsafe { CStr::from_ptr(s) }
        );
    }

    Ok(())
}
