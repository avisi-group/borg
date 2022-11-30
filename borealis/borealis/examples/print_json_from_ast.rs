use {color_eyre::eyre::Result, sail::ast::Ast, std::io};

fn main() -> Result<()> {
    color_eyre::install()?;

    let ast = bincode::deserialize_from::<_, Ast>(io::stdin().lock())?;
    serde_json::to_writer_pretty(io::stdout().lock(), &ast)?;

    Ok(())
}
