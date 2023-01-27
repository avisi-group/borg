#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("decoder_harness/include/aarch64-decode.h");
        include!("decoder_harness/include/wrapper.h");

        #[namespace = "captive::arch::aarch64"]
        type aarch64_decode;

        fn new_decoder() -> UniquePtr<aarch64_decode>;
    }
}
