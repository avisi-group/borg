use {color_eyre::eyre::Result, deepsize::DeepSizeOf, sail::ast::Ast, std::io};

fn main() -> Result<()> {
    color_eyre::install()?;

    let ast = bincode::deserialize_from::<_, Ast>(io::stdin().lock())?;

    println!("AST size: {} bytes", ast.deep_size_of());
    println!(
        "Interner size: {} bytes",
        sail::intern::INTERNER.current_memory_usage()
    );
    println!("Interner size: {} strings", sail::intern::INTERNER.len());

    Ok(())
}
