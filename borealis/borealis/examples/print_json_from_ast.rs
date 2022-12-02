use {
    color_eyre::eyre::Result,
    deepsize::DeepSizeOf,
    sail::ast::Ast,
    std::io::{self, Write},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let ast = bincode::deserialize_from::<_, Ast>(io::stdin().lock())?;

    writeln!(
        io::stderr().lock(),
        "AST size: {} bytes",
        ast.deep_size_of()
    )?;
    writeln!(
        io::stderr().lock(),
        "Interner size: {} bytes",
        sail::intern::INTERNER.current_memory_usage()
    )?;
    writeln!(
        io::stderr().lock(),
        "Interner size: {} strings",
        sail::intern::INTERNER.len()
    )?;

    serde_json::to_writer_pretty(io::stdout().lock(), &ast)?;

    Ok(())
}
