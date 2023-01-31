#[cxx::bridge]
pub mod ffi {
    // decode
    unsafe extern "C++" {
        include!("decoder_harness/include/arm64-decode.h");
        include!("decoder_harness/include/wrapper.h");

        #[namespace = "captive::arch::arm64"]
        pub type arm64_decode;

        pub fn new_decoder() -> UniquePtr<arm64_decode>;

        pub fn opcode(decoder: &arm64_decode) -> i32;

        pub unsafe fn decode(
            self: Pin<&mut arm64_decode>,
            isa_mode: u32,
            insn_pc: u64,
            ptr: *const u8,
        ) -> bool;
    }

    // disasm
    unsafe extern "C++" {
        include!("decoder_harness/include/arm64-disasm.h");
        include!("decoder_harness/include/wrapper.h");

        #[namespace = "captive::arch::arm64"]
        pub type arm64_disasm;

        pub fn new_disassembler() -> UniquePtr<arm64_disasm>;

        #[namespace = "captive::arch::arm64"]
        pub unsafe fn disassemble(
            self: Pin<&mut arm64_disasm>,
            pc: u64,
            data: *const u8,
        ) -> *const c_char;
    }
}
