use {
    clap::{self, Parser},
    clap_num::maybe_hex,
    std::{fs::File, path::PathBuf},
};

/// Reads a byte at a byte offset into a file
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the file
    file: PathBuf,
    /// Offset (in bytes)
    #[clap(value_parser=maybe_hex::<usize>)]
    offset: usize,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let Cli { file, offset } = Cli::parse();

    let f = File::open(file)?;

    let map = unsafe { memmap2::Mmap::map(&f) }?;

    println!("{offset:#x}: {:#x}", map[offset]);

    Ok(())
}
