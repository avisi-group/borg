use std::{fs, path::PathBuf, process::Command};

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("ocaml");

    // dune
    assert!(Command::new("dune")
        .current_dir(&root)
        .arg("build")
        .status()
        .unwrap()
        .success());

    let mut build = cc::Build::new();

    let search_dir = root.join("_build").join("default");

    fs::read_dir(search_dir)
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|p| p.extension().is_some())
        .filter(|p| p.extension().unwrap() == "o")
        .for_each(|p| {
            build.object(p);
        });

    build.compile("ocaml");
}
