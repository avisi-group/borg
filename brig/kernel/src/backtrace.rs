use {
    crate::{memory::PhysAddrExt, qemu_exit},
    core::{arch::asm, slice},
    elf::{
        endian::AnyEndian, parse::ParsingTable, string_table::StringTable, symbol::Symbol, ElfBytes,
    },
    spin::Once,
    x86_64::{PhysAddr, VirtAddr},
};

static SYMBOL_TABLE: Once<(
    VirtAddr,
    ParsingTable<'static, AnyEndian, Symbol>,
    StringTable<'static>,
)> = Once::INIT;

pub fn init(kernel_load_addr: VirtAddr, kernel_image_address: PhysAddr, kernel_image_len: usize) {
    let slice =
        unsafe { slice::from_raw_parts(kernel_image_address.to_virt().as_ptr(), kernel_image_len) };

    let elf = ElfBytes::<AnyEndian>::minimal_parse(slice).unwrap();
    let (sym_tab, str_tab) = elf.symbol_table().unwrap().unwrap();
    SYMBOL_TABLE.call_once(|| (kernel_load_addr, sym_tab, str_tab));

    unsafe { asm!("mov rbp, 0") };
}

#[repr(C)]
struct StackFrame {
    rbp: *const StackFrame,
    rip: u64,
}

pub fn backtrace() {
    log::error!("backtrace:");

    let mut stk: *const StackFrame;

    unsafe {
        asm!(
            "mov {0}, rbp",
            out(reg) stk
        )
    }

    while !stk.is_null() {
        let pc = VirtAddr::new(unsafe { &*stk }.rip);
        let symbol = rustc_demangle::demangle(lookup_symbol(pc));
        log::error!("    {:x} : {}", pc, symbol);
        unsafe { stk = (*stk).rbp };
    }

    qemu_exit();
}

fn lookup_symbol(pc: VirtAddr) -> &'static str {
    let (kernel_offset, sym_tab, str_tab) =
        SYMBOL_TABLE.get().expect("symbol table not initialized");

    // todo: check that it is within kernel range
    // todo: make backtrace struct and this a method on it
    if pc < *kernel_offset {
        return "???";
    }
    let pc = pc - *kernel_offset;

    sym_tab
        .iter()
        .find(|sym| pc >= sym.st_value && pc < (sym.st_value + sym.st_size))
        .map(|sym| str_tab.get(usize::try_from(sym.st_name).unwrap()).ok())
        .flatten()
        .unwrap_or("???")
}
