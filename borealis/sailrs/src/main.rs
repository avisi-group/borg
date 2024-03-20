use {
    clap::Parser,
    color_eyre::Result,
    common::{bytes, create_file, init_logger, intern::INTERNER},
    deepsize::DeepSizeOf,
    log::{info, trace},
    rkyv::ser::{serializers::AllocSerializer, Serializer},
    sailrs::load_from_config,
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
    init_logger(args.log.as_deref().unwrap_or(""))?;

    let (ast, jib) = load_from_config(args.input)?;

    trace!(
        "Size: AST {:.2} bytes, JIB {:.2} bytes",
        bytes(ast.deep_size_of()),
        bytes(jib.deep_size_of())
    );
    trace!(
        "INTERNER size: {:.2} bytes, {} strings",
        bytes(INTERNER.current_memory_usage()),
        INTERNER.len()
    );

    info!("serializing");
    let mut serializer = AllocSerializer::<0>::default();
    serializer.serialize_value(&jib).unwrap();
    let bytes = serializer.into_serializer().into_inner();
    create_file(args.output)?.write_all(&bytes)?;
    info!("done");

    Ok(())
}
