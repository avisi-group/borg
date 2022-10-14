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

    let source_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("sail_wrapper");
    let build_path = PathBuf::from(env::var("OUT_DIR")?).join("sail_wrapper");

    // dune
    ensure!(
        Command::new("dune")
            .arg("build")
            .env("DUNE_BUILD_DIR", &build_path)
            .current_dir(&source_path)
            .status()?
            .success(),
        format!("Failed to build OCaml wrapper library at source path {source_path:?} and build path {build_path:?}")
    );

    let mut build = cc::Build::new();

    let search_path = build_path.join("default");

    fs::read_dir(search_path)?
        .map(|f| f.unwrap().path())
        .filter(|p| p.extension().is_some())
        .filter(|p| p.extension().unwrap() == "o")
        .for_each(|p| {
            build.object(p);
        });

    build.compile("sail_wrapper");

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
