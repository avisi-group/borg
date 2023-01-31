fn main() {
    cxx_build::bridge("src/lib.rs")
        // compile decoder
        .file("include/arm64-decode.cpp")
        .file("include/arm64-disasm.cpp")
        // include dir
        .include("include")
        // flags
        .opt_level(3)
        .flag_if_supported("-std=gnu++20")
        .warnings(false)
        .compile("arm64");

    println!("cargo:rerun-if-changed=include");
}
