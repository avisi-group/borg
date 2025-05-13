use {
    borealis::{GenerationMode, parse_ir, sail_to_brig},
    clap::Parser,
    color_eyre::eyre::{Context, Result},
    errctx::PathCtx,
    log::info,
    std::{fs, path::PathBuf},
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

    /// Only generate IR - don't do codegen
    #[arg(long)]
    ir_only: bool,

    /// Path to Sail model archive
    input: PathBuf,
    /// Path to brig Rust file
    output: PathBuf,
}

/// Initialize the logger
pub fn init_logger(filters: &str) -> Result<()> {
    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder.parse_filters(filters);
    builder.try_init().wrap_err("Failed to initialise logger")?;
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    // set up the logger, defaulting to no output if the CLI flag was not supplied
    init_logger(args.log.as_deref().unwrap_or("info")).unwrap();

    let contents = fs::read_to_string(&args.input)
        .map_err(PathCtx::f(args.input))
        .unwrap();

    let jib = parse_ir(&contents);

    let mode = if let Some(ir_path) = args.dump_ir {
        std::fs::remove_dir_all(&ir_path).ok();
        if args.ir_only {
            GenerationMode::IrOnly(ir_path)
        } else {
            GenerationMode::CodeGenWithIr(ir_path)
        }
    } else {
        GenerationMode::CodeGen
    };

    sail_to_brig(jib, args.output, mode);

    info!("done");

    Ok(())
}
