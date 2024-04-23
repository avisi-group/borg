use {
    borealis::{brig::sail_to_brig, load_model},
    clap::Parser,
    color_eyre::eyre::Result,
    common::init_logger,
    log::info,
    std::path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Logging filter string (e.g. "borealis=debug" or "trace")
    #[arg(long)]
    log: Option<String>,

    /// Writes all intermediate representations to disk in the specified folder
    #[arg(long)]
    dump_ir: Option<PathBuf>,

    /// Path to Sail model archive
    input: PathBuf,
    /// Path to brig Rust file
    output: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    // set up the logger, defaulting to no output if the CLI flag was not supplied
    init_logger(args.log.as_deref().unwrap_or("info")).unwrap();

    sail_to_brig(
        load_model(&args.input).into_iter(),
        args.output,
        args.dump_ir,
    );

    info!("done");

    Ok(())
}
