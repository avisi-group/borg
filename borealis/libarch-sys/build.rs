use std::{env, path::PathBuf};

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=./include");

    cc::Build::new()
        .cpp(true)
        .flag("-std=gnu++20")
        .flag("-xc++")
        .file("./include/arm64-decode.cpp")
        .file("./include/arm64-disasm.cpp")
        .file("./include/wrapper.cpp")
        .include("./include")
        .warnings(false)
        .compile("libarch");

    let bindings = bindgen::Builder::default()
        // clang arguments
        .clang_arg("-I./include")
        .clang_arg("-std=gnu++20")
        .clang_arg("-xc++")
        // The input header we would like to generate
        // bindings for.
        .header("./include/arm64-decode.h")
        .header("./include/arm64-disasm.h")
        .header("./include/wrapper.h")
        // allowlisted items
        .allowlist_type("captive.*")
        .allowlist_function("captive.*")
        .allowlist_var("captive.*")
        .allowlist_file("./include/wrapper.h")
        .enable_cxx_namespaces()
        .vtable_generation(true)
        .generate_block(true)
        .rustified_enum(".*arm64_decode_arm64_opcodes.*")
        .layout_tests(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
