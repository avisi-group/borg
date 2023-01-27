fn main() {
    cxx_build::bridge("src/lib.rs")
        .include("include")
        .file("include/aarch64-decode.cpp")
        .compile("aarch64-decode");
}
