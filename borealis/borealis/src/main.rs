use {
    borealis::{deserialize_compressed_ast, genc, sail_to_genc},
    clap::{Parser, Subcommand},
    color_eyre::eyre::{bail, Result, WrapErr},
    common::intern::INTERNER,
    deepsize::DeepSizeOf,
    errctx::PathCtx,
    log::{info, trace, warn},
    lz4_flex::frame::{BlockMode, FrameEncoder as Lz4Encoder, FrameInfo},
    sail::load_from_config,
    std::{
        ffi::OsStr,
        fs::File,
        io::{BufReader, BufWriter},
        path::{Path, PathBuf},
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Logging filter string (e.g. "borealis=debug" or "trace")
    #[arg(long)]
    log: Option<String>,

    /// Path to Sail JSON config or bincode AST
    #[arg(short)]
    input: PathBuf,

    /// Warning! Disables checking that output directory is empty or output file does not exist before writing.
    #[arg(long)]
    force: bool,

    /// Output format
    #[command(subcommand)]
    output: Output,
}

#[derive(Subcommand, Debug)]
enum Output {
    /// Output a GenC description of the instruction set architecture
    Genc {
        /// Path to empty folder where GenC description files will be emitted.
        #[arg(short)]
        output: PathBuf,
    },
    /// Serialize the AST to JSON
    Json {
        /// Path to JSON file.
        #[arg(short)]
        output: PathBuf,
    },
    /// Serialize the AST to Bincode
    Bincode {
        /// Path to Bincode file.
        #[arg(short)]
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

    // either parse AST from Sail config file or deserialize AST from bincode
    let (ast, jib) = match args.input.extension().and_then(OsStr::to_str) {
        Some("json") => {
            info!("Loading Sail config {:?}", args.input);
            load_from_config(args.input).wrap_err("Failed to load Sail files")?
        }
        Some("lz4") => {
            info!("Deserializing compressed bincode {:?}", args.input);
            deserialize_compressed_ast(BufReader::new(File::open(args.input)?))?
        }
        _ => bail!("Unrecognised input format {:?}", args.input),
    };

    trace!(
        "Size: AST {} bytes, JIB {} bytes",
        ast.deep_size_of(),
        jib.deep_size_of()
    );
    trace!(
        "INTERNER size: {} bytes, {} strings",
        INTERNER.current_memory_usage(),
        INTERNER.len()
    );

    match args.output {
        Output::Genc { output } => {
            info!("Converting Sail AST to GenC");
            let description = sail_to_genc(&ast, &jib);

            info!("Exporting GenC description");
            genc::export(&description, output, args.force)
                .wrap_err("Error while exporting GenC description")?
        }

        Output::Json { output } => {
            info!("Serializing AST to JSON");
            serde_json::to_writer_pretty(
                BufWriter::new(create_file(output, args.force)?),
                &(ast, jib),
            )?
        }

        Output::Bincode { output } => {
            info!("Serializing AST to compressed bincode");

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
/// If the file at the supplied path already exists and `force` is true it will be overwritten, otherwise an error will be returned.
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
