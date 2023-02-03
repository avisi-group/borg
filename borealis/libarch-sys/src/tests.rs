use {
    crate::root::{
        captive::arch::arm64::{arm64_decode_decode, arm64_disasm_disassemble},
        destruct_decoder, destruct_disassembler, new_decoder, new_disassembler,
    },
    std::ffi::{c_void, CStr},
};

#[test]
fn single_decode() {
    let data = [0x1D, 0x00, 0x80, 0xD2];

    let decoder = unsafe { new_decoder() };
    let disassembler = unsafe { new_disassembler() };

    unsafe { arm64_decode_decode(decoder as *mut c_void, 0, 0, &data[0]) };

    let str_ptr =
        unsafe { arm64_disasm_disassemble(disassembler as *mut c_void, 0, decoder as *const u8) };

    assert_eq!(
        unsafe { CStr::from_ptr(str_ptr) }.to_str().unwrap(),
        "branch_unconditional_immediate_decode1"
    );

    unsafe { destruct_decoder(decoder) };
    unsafe { destruct_disassembler(disassembler) };
}
