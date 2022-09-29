use {
    ariadne::Source,
    chumsky::Parser as _,
    color_eyre::eyre::{Context, Result},
    sail::error::Error,
    std::env,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = env::args();

    // parse each file
    for path in args.skip(1) {
        let source = std::fs::read_to_string(&path).wrap_err("Failed to read input source file")?;

        match sail::lexer().parse(source.as_str()) {
            Ok(ast) => println!("{:#?}", ast),
            Err(errors) => errors.iter().map(Error::into_report).for_each(|report| {
                report.print(Source::from(&source)).unwrap();
            }),
        }
    }

    Ok(())
}
