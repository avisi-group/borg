use {
    color_eyre::Result,
    errctx::PathCtx,
    libarch_sys,
    memmap2::Mmap,
    std::{env::args, fs::File},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let path = args().skip(1).next().unwrap();

    let file = File::open(&path).map_err(PathCtx::f(&path)).unwrap();
    let mmap = unsafe { Mmap::map(&file) }.unwrap();

    println!("{}", libarch_sys::disassemble(&mmap));

    Ok(())
}
