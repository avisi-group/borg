use {
    borealis::genc::Description,
    clap::Parser,
    color_eyre::eyre::{Result, WrapErr},
    std::path::PathBuf,
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
    Description::empty()
        .export(args.out_dir, args.force)
        .wrap_err("Error while exporting GenC description")?;

    Ok(())
}