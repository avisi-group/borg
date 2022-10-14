use {
    color_eyre::eyre::{Context, Result},
    sail::parser::load_files,
    std::env,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = env::args();

    load_files(args.skip(1).collect::<Vec<_>>()).wrap_err("Failed to parse Sail files")?;

    Ok(())
}
