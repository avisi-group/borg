fn main() {
    cxx_build::bridge("src/lib.rs")
        // compile decoder
        .file("include/aarch64-decode.cpp")
        // include dir
        .include("include")
        // flags
        .opt_level(3)
        .flag_if_supported("-std=gnu++20")
        .warnings(true)
        .compile("aarch64-decode");

    println!("cargo:rerun-if-changed=include");
}
