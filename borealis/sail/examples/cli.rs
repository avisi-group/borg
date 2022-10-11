use {
    color_eyre::eyre::{Context, Result},
    std::env,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = env::args();

    // parse each file
    for path in args.skip(1) {
        let source = std::fs::read_to_string(&path).wrap_err("Failed to read input source file")?;

        dbg!(source);
    }

    Ok(())
}
