use {
    crate::{arch::x86::memory::PhysAddrExt, qemu_exit},
    core::{arch::asm, slice},
    elf::{
        endian::AnyEndian, parse::ParsingTable, string_table::StringTable, symbol::Symbol, ElfBytes,
    },
    spin::Once,
    x86_64::{PhysAddr, VirtAddr},
};

static BACKTRACER: Once<Backtracer> = Once::INIT;

struct Backtracer {
    kernel_image_virt_addr: VirtAddr,
    kernel_image_len: usize,
    symbol_table: ParsingTable<'static, AnyEndian, Symbol>,
    string_table: StringTable<'static>,
}

impl Backtracer {
    // TODO: explain why the kernel image addresses are different
    fn new(
        kernel_image_virt_addr: VirtAddr,
        kernel_image_phys_addr: PhysAddr,
        kernel_image_len: usize,
    ) -> Self {
        let elf_slice = unsafe {
            slice::from_raw_parts(kernel_image_phys_addr.to_virt().as_ptr(), kernel_image_len)
        };
        let elf = ElfBytes::<AnyEndian>::minimal_parse(elf_slice).unwrap();
        let (symbol_table, string_table) = elf.symbol_table().unwrap().unwrap();

        Self {
            kernel_image_virt_addr,
            kernel_image_len,
            symbol_table,
            string_table,
        }
    }

    fn lookup_symbol(&self, pc: VirtAddr) -> &'static str {
        // todo: check that it is within kernel range
        // todo: make backtrace struct and this a method on it
        if !(self.kernel_image_virt_addr..self.kernel_image_virt_addr + self.kernel_image_len)
            .contains(&pc)
        {
            return "???";
        }

        let pc = pc - self.kernel_image_virt_addr;

        self.symbol_table
            .iter()
            .find(|sym| pc >= sym.st_value && pc < (sym.st_value + sym.st_size))
            .map(|sym| {
                self.string_table
                    .get(usize::try_from(sym.st_name).unwrap())
                    .ok()
            })
            .flatten()
            .unwrap_or("???")
    }
}

pub fn init(kernel_load_addr: VirtAddr, kernel_image_address: PhysAddr, kernel_image_len: usize) {
    BACKTRACER
        .call_once(|| Backtracer::new(kernel_load_addr, kernel_image_address, kernel_image_len));

    // push null to base pointer to prevent recursing indefinitely
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
        let symbol = rustc_demangle::demangle(BACKTRACER.get().unwrap().lookup_symbol(pc));
        log::error!("    {:x} : {}", pc, symbol);
        unsafe { stk = (*stk).rbp };
    }

    qemu_exit();
}
