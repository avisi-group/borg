use {
    borealis::genc::{export, Description},
    clap::Parser,
    color_eyre::eyre::{Result, WrapErr},
    deepsize::DeepSizeOf,
    std::{
        io::{self, Write},
        path::PathBuf,
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to Sail JSON config
    #[arg(short)]
    input: PathBuf,

    /// Path to empty folder where GenC description files will be emitted.
    #[arg(short)]
    output: PathBuf,

    #[arg(long)]
    /// Warning! Disables checking that output directory is empty before writing files.
    force: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    let (_name, ast, _env) =
        sail::load_from_config(args.input).wrap_err("Failed to load Sail files")?;

    export(&Description::empty(), args.output, args.force)
        .wrap_err("Error while exporting GenC description")?;

    writeln!(
        io::stderr().lock(),
        "AST occupies {} bytes",
        ast.deep_size_of()
    )?;
    writeln!(
        io::stderr().lock(),
        "Interner size: {} bytes",
        sail::intern::INTERNER.current_memory_usage()
    )?;
    writeln!(
        io::stderr().lock(),
        "Interner size: {} strings",
        sail::intern::INTERNER.len()
    )?;

    bincode::serialize_into(io::stdout().lock(), &ast)?;

    Ok(())
}
