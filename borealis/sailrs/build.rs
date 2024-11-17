use {
    color_eyre::{
        eyre::{ensure, WrapErr},
        Result,
    },
    std::{
        env, fs,
        path::PathBuf,
        process::{Command, Stdio},
    },
};

const PROJECT_NAME: &str = "wrapper";

fn main() -> Result<()> {
    color_eyre::install()?;

    check_build_environment().wrap_err("Checking build environment")?;

    let source_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join(PROJECT_NAME);
    let build_path = PathBuf::from(env::var("OUT_DIR")?).join(PROJECT_NAME);

    // dune
    ensure!(Command::new("dune").arg("build").arg("--profile").arg("release").env("DUNE_BUILD_DIR", &build_path).current_dir(&source_path).status().wrap_err("Failed to execute dune build")?.success(), format!("Failed to build OCaml wrapper library at source path {source_path:?} and build path {build_path:?}"));

    let mut build = cc::Build::new();

    let search_path = build_path.join("default");

    fs::read_dir(&search_path)
        .wrap_err(format!("Failed to read dir {:?}", &search_path))?
        .map(|f| f.unwrap().path())
        .filter(|p| p.extension().is_some())
        .filter(|p| p.extension().unwrap() == "o")
        .for_each(|p| {
            build.object(p);
        });

    build.compile(PROJECT_NAME);

    // Fix linker errors on aarch64 macOS
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    {
        println!("cargo:rustc-link-search=/opt/homebrew/lib");
    }

    println!("cargo:rustc-link-lib=gmp");

    // rebuild if OCaml package is modified
    println!("cargo:rerun-if-changed=wrapper");

    Ok(())
}

/// Checks that opam and dune binaries are present
fn check_build_environment() -> Result<()> {
    ensure!(
        Command::new("opam")
            .arg("--version")
            .stdout(Stdio::null())
            .status()
            .wrap_err("Failed to execute opam, is it installed and in the PATH?")?
            .success(),
        "opam did not exit successfully"
    );

    ensure!(
        Command::new("dune")
            .arg("--version")
            .stdout(Stdio::null())
            .status()
            .wrap_err("Failed to execute dune, is it installed and in the PATH?")?
            .success(),
        "dune did not exit successfully"
    );

    Ok(())
}
