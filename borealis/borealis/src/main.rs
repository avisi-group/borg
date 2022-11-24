use {
    borealis::genc::{export, Description},
    clap::Parser,
    color_eyre::eyre::{Result, WrapErr},
    std::path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to empty folder where GenC description files will be emitted.
    #[arg(short)]
    output: PathBuf,

    #[arg(long)]
    /// Warning! Disables checking that output directory is empty before writing files.
    force: bool,

    /// Path(s) to Sail files
    #[arg(
        short,
        required = true,
        use_value_delimiter = true,
        value_delimiter = ' ',
        num_args(0..)
    )]
    input: Vec<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    export(&Description::empty(), args.output, args.force)
        .wrap_err("Error while exporting GenC description")?;

    dbg!(sail::load_files(dbg!(args.input)).wrap_err("Failed to parse Sail files")?);

    Ok(())
}
