#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("decoder_harness/include/aarch64-decode.h");
        include!("decoder_harness/include/wrapper.h");

        #[namespace = "captive::arch::aarch64"]
        pub type aarch64_decode;

        pub fn new_decoder() -> UniquePtr<aarch64_decode>;

        pub fn opcode(decoder: &aarch64_decode) -> u64;

        pub unsafe fn decode(
            self: Pin<&mut aarch64_decode>,
            isa_mode: u32,
            insn_pc: u64,
            ptr: *const u8,
        ) -> bool;
    }
}
