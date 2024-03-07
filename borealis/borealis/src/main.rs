use {
    borealis::{
        genc::{self, sail_to_genc},
        load_sail,
        rust::sail_to_brig,
    },
    clap::{Parser, Subcommand},
    color_eyre::eyre::{Result, WrapErr},
    errctx::PathCtx,
    log::info,
    lz4_flex::frame::{BlockMode, FrameEncoder as Lz4Encoder, FrameInfo},
    std::{
        fs::File,
        io::Write,
        path::{Path, PathBuf},
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Logging filter string (e.g. "borealis=debug" or "trace")
    #[arg(long)]
    log: Option<String>,

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

    /// Compile a Sail ISA specification to a Rust module for use with brig
    Sail2brig {
        /// Produce a standalone (no brig traits) Rust source file for testing
        /// purposes
        #[arg(long)]
        standalone: bool,
        /// Path to Sail JSON config or bincode AST
        input: PathBuf,
        /// Path to Rust file.
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    // set up the logger, defaulting to no output if the CLI flag was not supplied
    init_logger(args.log.as_deref().unwrap_or(""))?;

    match args.command {
        Command::Sail2genc { input, output } => {
            let (ast, jib) = load_sail(input)?;

            info!("Converting Sail model to GenC");
            let description = sail_to_genc(&ast, &jib);

            info!("Exporting GenC description");
            genc::export(&description, output).wrap_err("Error while exporting GenC description")?
        }
        Command::Sail2bincode { input, output } => {
            let (ast, jib) = load_sail(input)?;

            info!("Serializing Sail model to compressed bincode");

            let mut frame_info = FrameInfo::new();
            frame_info.block_mode = BlockMode::Linked;
            let mut encoder = Lz4Encoder::with_frame_info(frame_info, create_file(output)?);
            bincode::serialize_into(&mut encoder, &(ast, jib))?;
            encoder.finish()?;
            info!("done");
        }
        Command::Sail2brig {
            input,
            output,
            standalone,
        } => {
            let (ast, jib) = load_sail(input)?;

            info!("Converting Sail model to brig module");
            let mut writer = File::create(output)?;
            let tokens = sail_to_brig(&ast, &jib, standalone);

            let syntax_tree =
                syn::parse_file(&tokens.to_string()).wrap_err("failed to parse tokens")?;
            let formatted = prettyplease::unparse(&syntax_tree);
            writer.write_all(formatted.as_bytes())?;

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
/// If the file at the supplied path already exists it will
/// be overwritten.
fn create_file<P: AsRef<Path>>(path: P) -> Result<File> {
    File::options()
        .write(true) // we want to write to the file...
        .create(true) // ...creating if it does not exist..
        .truncate(true) // ...and truncate before writing
        .open(path.as_ref())
        .map_err(PathCtx::f(path))
        .wrap_err("Failed to write to file")
}
