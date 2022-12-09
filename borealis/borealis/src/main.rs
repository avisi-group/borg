use {
    borealis::genc::{export, Description},
    clap::{Parser, Subcommand},
    color_eyre::eyre::{bail, Result, WrapErr},
    common::{identifiable::unique_id, intern::INTERNER},
    deepsize::DeepSizeOf,
    log::{debug, trace},
    sail::dot,
    std::{
        ffi::OsStr,
        fs::File,
        io::{self, BufReader},
        path::PathBuf,
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

    #[command(subcommand)]
    output: Output,
}

#[derive(Subcommand, Debug)]
enum Output {
    Genc {
        /// Path to empty folder where GenC description files will be emitted.
        #[arg(short)]
        output: PathBuf,

        #[arg(long)]
        /// Warning! Disables checking that output directory is empty before writing files.
        force: bool,
    },
    Dot,
    Json,
    Bincode,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    // set up the logger, defaulting to no output if the CLI flag was not supplied
    init_logger(args.log.as_deref().unwrap_or(""))?;

    // either parse AST from Sail config file or deserialize AST from bincode
    let ast = match args.input.extension().and_then(OsStr::to_str) {
        Some("json") => {
            debug!("Loading Sail config {:?}", args.input);
            sail::load_from_config(args.input)
                .wrap_err("Failed to load Sail files")?
                .1
        }
        Some("bincode") => {
            debug!("Deserializing bincode {:?}", args.input);
            bincode::deserialize_from(BufReader::new(File::open(args.input)?))?
        }
        _ => bail!("Unrecognised input format {:?}", args.input),
    };

    trace!(
        "AST size: {} bytes, ≈{} nodes",
        ast.deep_size_of(),
        unique_id(),
    );
    trace!(
        "INTERNER size: {} bytes, {} strings",
        INTERNER.current_memory_usage(),
        INTERNER.len()
    );

    match args.output {
        Output::Genc { output, force } => {
            debug!("Compiling to GenC");
            export(&Description::from(&ast), output, force)
                .wrap_err("Error while exporting GenC description")?
        }
        Output::Dot => {
            debug!("Printing AST as do graph");
            dot::render(&ast, &mut io::stdout().lock())?
        }
        Output::Json => {
            debug!("Serializing AST to JSON");
            serde_json::to_writer_pretty(io::stdout().lock(), &ast)?
        }
        Output::Bincode => {
            debug!("Serializing AST to bincode");
            bincode::serialize_into(io::stdout().lock(), &ast)?
        }
    }

    Ok(())
}

fn init_logger(filters: &str) -> Result<()> {
    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder.parse_filters(filters);
    builder.try_init()?;
    Ok(())
}
