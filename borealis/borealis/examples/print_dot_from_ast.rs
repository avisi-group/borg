use {
    color_eyre::eyre::Result,
    deepsize::DeepSizeOf,
    sail::{ast::Ast, dot},
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

    dot::render(&ast, &mut io::stdout().lock())?;

    Ok(())
}
