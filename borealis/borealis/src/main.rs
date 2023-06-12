use {
    borealis::{genc, load_sail, sail_to_genc},
    clap::{Parser, Subcommand},
    color_eyre::eyre::{Result, WrapErr},
    errctx::PathCtx,
    log::{info, warn},
    lz4_flex::frame::{BlockMode, FrameEncoder as Lz4Encoder, FrameInfo},
    std::{
        fs::File,
        path::{Path, PathBuf},
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Logging filter string (e.g. "borealis=debug" or "trace")
    #[arg(long)]
    log: Option<String>,

    /// Warning! Disables checking that output directory is empty or output file
    /// does not exist before writing.
    #[arg(long)]
    force: bool,

    /// Borealis command
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Compile a Sail ISA specification to GenC
    Sail2genc {
        /// Path to Sail JSON config or bincode AST
        input: PathBuf,
        /// Path to empty folder where GenC description files will be emitted.
        output: PathBuf,
    },

    /// Serialize a Sail ISA specification to a compressed bincode file
    Sail2bincode {
        /// Path to Sail JSON config or bincode AST
        input: PathBuf,
        /// Path to Bincode file.
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    warn!("Force flag set! May result in data being overwritten!");

    // set up the logger, defaulting to no output if the CLI flag was not supplied
    init_logger(args.log.as_deref().unwrap_or(""))?;

    match args.command {
        Command::Sail2genc { input, output } => {
            let (ast, jib) = load_sail(input)?;

            info!("Converting Sail model to GenC");
            let description = sail_to_genc(&ast, &jib);

            info!("Exporting GenC description");
            genc::export(&description, output, args.force)
                .wrap_err("Error while exporting GenC description")?
        }
        Command::Sail2bincode { input, output } => {
            let (ast, jib) = load_sail(input)?;

            info!("Serializing Sail model to compressed bincode");

            let mut frame_info = FrameInfo::new();
            frame_info.block_mode = BlockMode::Linked;
            let mut encoder =
                Lz4Encoder::with_frame_info(frame_info, create_file(output, args.force)?);
            bincode::serialize_into(&mut encoder, &(ast, jib))?;
            encoder.finish()?;
            info!("done");
        }
    }

    Ok(())
}

/// Initialize the logger
fn init_logger(filters: &str) -> Result<()> {
    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder.parse_filters(filters);
    builder.try_init().wrap_err("Failed to initialise logger")?;
    Ok(())
}

/// Creates the file supplied in `path`.
///
/// If the file at the supplied path already exists and `force` is true it will
/// be overwritten, otherwise an error will be returned.
fn create_file<P: AsRef<Path>>(path: P, force: bool) -> Result<File> {
    File::options()
        .write(true) // we want to write to the file
        .create_new(!force) // fail if it already exists and force is true...
        .create(true) // ...otherwise create...
        .truncate(true) // ...and truncate before writing
        .open(path.as_ref())
        .map_err(PathCtx::f(path))
        .wrap_err(format!("Failed to write to file, force = {force}"))
}
