use {
    color_eyre::{eyre::ensure, Result},
    std::{
        env, fs,
        path::PathBuf,
        process::{Command, Stdio},
    },
};

fn main() -> Result<()> {
    color_eyre::install()?;

    check_build_environment()?;

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("ocaml");

    // dune
    assert!(Command::new("dune")
        .current_dir(&root)
        .arg("build")
        .status()?
        .success());

    let mut build = cc::Build::new();

    let search_dir = root.join("_build").join("default");

    fs::read_dir(search_dir)?
        .map(|f| f.unwrap().path())
        .filter(|p| p.extension().is_some())
        .filter(|p| p.extension().unwrap() == "o")
        .for_each(|p| {
            build.object(p);
        });

    build.compile("ocaml");

    Ok(())
}

/// Checks that opam and dune binaries are present
fn check_build_environment() -> Result<()> {
    ensure!(
        Command::new("opam")
            .arg("--version")
            .stdout(Stdio::null())
            .status()?
            .success(),
        "Failed to execute opam, is it installed and in the PATH?"
    );

    ensure!(
        Command::new("dune")
            .arg("--version")
            .stdout(Stdio::null())
            .status()?
            .success(),
        "Failed to execute dune, is it installed and in the PATH?"
    );

    Ok(())
}
