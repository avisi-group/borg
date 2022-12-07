use {
    borealis::genc::{export, Description},
    clap::{Parser, Subcommand},
    color_eyre::eyre::{bail, Result, WrapErr},
    deepsize::DeepSizeOf,
    sail::dot,
    std::{
        ffi::OsStr,
        fs::File,
        io::{self, BufReader, Write},
        path::PathBuf,
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

    // either parse AST from Sail config file or deserialize AST from bincode
    let ast = match args.input.extension().and_then(OsStr::to_str) {
        Some("json") => {
            sail::load_from_config(args.input)
                .wrap_err("Failed to load Sail files")?
                .1
        }
        Some("bincode") => {
            let file = File::open(args.input)?;
            let reader = BufReader::new(file);
            bincode::deserialize_from(reader)?
        }
        _ => bail!("Unrecognised input format {:?}", args.input),
    };

    writeln!(
        io::stderr().lock(),
        "AST occupies {} bytes",
        ast.deep_size_of()
    )?;
    writeln!(
        io::stderr().lock(),
        "Interner size: {} bytes",
        common::intern::INTERNER.current_memory_usage()
    )?;
    writeln!(
        io::stderr().lock(),
        "Interner size: {} strings",
        common::intern::INTERNER.len()
    )?;
    writeln!(
        io::stderr().lock(),
        "Counter: {} nodes",
        common::identifiable::unique_id()
    )?;

    match args.output {
        Output::Genc { output, force } => export(&Description::empty(), output, force)
            .wrap_err("Error while exporting GenC description")?,
        Output::Dot => dot::render(&ast, &mut io::stdout().lock())?,
        Output::Json => serde_json::to_writer_pretty(io::stdout().lock(), &ast)?,
        Output::Bincode => bincode::serialize_into(io::stdout().lock(), &ast)?,
    }

    Ok(())
}
