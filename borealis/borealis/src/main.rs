use borealis::genc::{Bank, RegisterSpace};

use {
    borealis::genc::{self},
    clap::Parser,
    color_eyre::eyre::{Result, WrapErr},
    std::{collections::HashMap, path::PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to empty folder where GenC description files will be emitted.
    #[arg(short, long)]
    out_dir: PathBuf,

    #[arg(long)]
    /// Warning! Disables checking that output directory is empty before writing files.
    force: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    genc::Description {
        name: "arm64".to_owned(),
        endianness: genc::Endianness::LittleEndian,
        wordsize: 64,
        fetchsize: 32,
        registers: vec![RegisterSpace {
            size: 248,
            banks: vec![Bank {
                name: "RBX".to_owned(),
                typ: genc::Typ::Uint64,
                offset: 0,
                count: 31,
                stride: 8,
                element_count: 1,
                element_size: 8,
                element_stride: 8,
            }],
        }],
        instructions: HashMap::new(),
    }
    .export(args.out_dir, args.force)
    .wrap_err("Error while exporting GenC description")?;

    Ok(())
}
