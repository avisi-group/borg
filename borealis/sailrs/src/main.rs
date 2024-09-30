use {
    clap::Parser,
    color_eyre::Result,
    common::intern,
    common::{intern::get_interner_state, HashMap},
    deepsize::DeepSizeOf,
    log::info,
    rkyv::ser::{serializers::AllocSerializer, Serializer},
    sailrs::{bytes, create_file_buffered, init_logger, load_from_config},
    std::{io::Write, path::PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Logging filter string (e.g. "borealis=debug" or "trace")
    #[arg(long)]
    log: Option<String>,

    /// Sail model JSON path
    input: PathBuf,

    /// Archive output path
    output: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    // set up the logger, defaulting to no output if the CLI flag was not supplied
    init_logger(args.log.as_deref().unwrap_or("info"))?;

    intern::init(HashMap::default());

    let jib = load_from_config(args.input)?;

    info!("JIB size: {:.2} bytes", bytes(jib.deep_size_of()));

    let state = (jib, get_interner_state());

    info!("serializing");

    let mut serializer = AllocSerializer::<16384>::default();
    serializer.serialize_value(&state).unwrap();
    let bytes = serializer.into_serializer().into_inner();
    create_file_buffered(&args.output)?.write_all(&bytes)?;

    info!("done");

    Ok(())
}
