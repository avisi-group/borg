use {
    clap::{self, Parser},
    clap_num::maybe_hex,
    std::{fs::File, path::PathBuf},
};

/// Writes a byte at a byte offset into a file
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the file
    file: PathBuf,
    /// Offset (in bytes)
    #[clap(value_parser=maybe_hex::<usize>)]
    offset: usize,
    /// Byte to write (alright!)
    #[clap(value_parser=maybe_hex::<u8>)]
    byte: u8,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Cli { file, offset, byte } = Cli::parse();

    let f = File::options().write(true).read(true).open(file)?;

    let mut map = unsafe { memmap2::MmapMut::map_mut(&f) }?;

    let prev = map[offset];
    map[offset] = byte;
    f.sync_all()?;

    println!("{offset:#x}: {prev:#x} -> {byte:#x}");

    Ok(())
}
