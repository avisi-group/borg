use {
    ariadne::Source,
    chumsky::{Parser as _, Stream},
    color_eyre::eyre::{Context, Result},
    sail::{error::Error, span::Span},
    std::env,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    // parse command line arguments
    let args = env::args();

    // parse each file
    for path in args.skip(1) {
        let source = std::fs::read_to_string(&path).wrap_err("Failed to read input source file")?;

        let (tokens, errors) = sail::lexer().parse_recovery(source.as_str());

        if let Some(tokens) = tokens {
            let len = source.chars().count();
            let stream = Stream::from_iter(Span::from(len..len + 1), tokens.into_iter());
            sail::parser().parse_recovery(stream);
        }

        errors.iter().map(Error::into_report).for_each(|report| {
            report.print(Source::from(&source)).unwrap();
        });
    }

    Ok(())
}
