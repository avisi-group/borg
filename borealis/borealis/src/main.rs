use {
    borealis::run, clap::Parser, color_eyre::eyre::Result, common::init_logger, log::info,
    std::path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Logging filter string (e.g. "borealis=debug" or "trace")
    #[arg(long)]
    log: Option<String>,

    /// Produce a standalone (no brig traits) Rust source file for testing
    /// purposes
    #[arg(long)]
    standalone: bool,
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
    init_logger(args.log.as_deref().unwrap_or("")).unwrap();

    run(args.input, args.output, args.standalone);

    info!("done");

    Ok(())
}
