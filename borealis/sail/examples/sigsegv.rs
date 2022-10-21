use {color_eyre::Result, sail::parser::load_files};

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    simple_logger::init_with_env().unwrap();

    println!(
        "{:#?}",
        load_files(vec![
            "/Users/ferdiamckeogh/Downloads/borealis/sail/examples/simple.sail".to_owned()
        ])?
        .1
        .defs
    );

    Ok(())
}
