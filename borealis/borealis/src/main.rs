use {
    borealis::genc::{export, Description},
    clap::Parser,
    color_eyre::eyre::{Result, WrapErr},
    std::{io, path::PathBuf},
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

    serde_json::to_writer_pretty(io::stdout().lock(), &ast).unwrap();

    Ok(())
}
