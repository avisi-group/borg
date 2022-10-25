use {
    color_eyre::eyre::{Context, Result},
    sail::parser::load_files,
    std::env,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    simple_logger::init_with_env()?;

    // parse command line arguments
    let args = env::args();

    let (_name, ast, _env) =
        load_files(args.skip(1).collect::<Vec<_>>()).wrap_err("Failed to parse Sail files")?;

    dbg!(ast);

    Ok(())
}
