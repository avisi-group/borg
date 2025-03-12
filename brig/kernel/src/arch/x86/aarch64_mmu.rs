use {
    crate::{arch::x86::memory::guest_physical_to_host_virt, dbt::models::ModelDevice},
    aarch64_paging::paging::Descriptor,
    iced_x86::code_asm::zmm,
};

// returns guest physical address
pub fn guest_translate(device: &ModelDevice, guest_virtual_address: u64) -> Option<u64> {
    let tcr_el1 = *device.get_register_mut::<u64>("TCR_EL1_bits");
    log::trace!("tcr_el1: {tcr_el1:x}");
    // let ttbcr = *device.get_register_mut::<u32>("TTBCR_S_bits");
    // panic!("{ttbcr:032b}");
    // let ttbcr_n = ttbcr & 0b111;
    let ttbr0_el1 = *device.get_register_mut::<u64>("_TTBR0_EL1_bits");
    let ttbr1_el1 = *device.get_register_mut::<u64>("_TTBR1_EL1_bits");
    log::trace!("ttbr0_el1: {ttbr0_el1:x}");
    log::trace!("ttbr1_el1: {ttbr1_el1:x}");

    let translation_table_base_guest_phys = match guest_virtual_address {
        // because we masked off in emitter.rs:write_memory
        // todo: check this
        ..=0x0000_007F_FFFF_FFFF => ttbr0_el1,
        0x0000_0080_0000_0000.. => ttbr1_el1,
        // addr => todo!("fault at {addr:x}"),
    };

    let ttbgp_masked = translation_table_base_guest_phys & !0xffff000000000fff;

    log::trace!("guest_virtual_address: {guest_virtual_address:x?}");
    log::trace!("translation_table_base_guest_phys: {translation_table_base_guest_phys:x?}");
    log::trace!("ttbgp_masked: {ttbgp_masked:x?}");

    let translation_table_base = guest_physical_to_host_virt(ttbgp_masked);
    log::trace!("translation_table_base: {translation_table_base:x?}");
    let table = unsafe { &*(translation_table_base.as_ptr::<[Descriptor; 512]>()) };

    //log::trace!("table: {table:x?}");

    // Skip L0, because 3-level page tables.
    translate_l1(device, table, guest_virtual_address)
}

fn translate_l0(
    device: &ModelDevice,
    table: &[Descriptor; 512],
    guest_virtual_address: u64,
) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 39) & 0x1ff) as usize;

    log::trace!("entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::trace!("entry: {entry:x?}");

    if entry.is_table_or_page() {
        translate_l1(device, entry_to_table(&entry), guest_virtual_address)
    } else {
        panic!("entry was not table or page")
    }
}

fn translate_l1(
    device: &ModelDevice,
    table: &[Descriptor; 512],
    guest_virtual_address: u64,
) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 30) & 0x1ff) as usize;
    log::trace!("l1 entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::trace!("l1 entry: {entry:x?}");

    if !entry.is_valid() {
        // guest page fault, look up exception vector table (VBAR_EL2)
        guest_page_fault(device);
        panic!("invalid")
    }

    if entry.is_table_or_page() {
        translate_l2(device, entry_to_table(&entry), guest_virtual_address)
    } else {
        todo!("block")
    }
}

fn translate_l2(
    device: &ModelDevice,
    table: &[Descriptor; 512],
    guest_virtual_address: u64,
) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 21) & 0x1ff) as usize;
    log::trace!("l2 entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::trace!("l2 entry: {entry:x?}");

    if !entry.is_valid() {
        panic!("invalid")
    }

    if entry.is_table_or_page() {
        translate_l3(device, entry_to_table(&entry), guest_virtual_address)
    } else {
        Some((entry.output_address().0 as u64) | (guest_virtual_address & ((1 << 21) - 1)))
    }
}

fn translate_l3(
    device: &ModelDevice,
    table: &[Descriptor; 512],
    guest_virtual_address: u64,
) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 21) & 0x1ff) as usize;
    log::trace!("l3 entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::trace!("l3 entry: {entry:x?}");
    if entry.is_valid() && entry.is_table_or_page() {
        panic!("{entry:x?}")
    } else {
        panic!("invalid")
    }
}

fn entry_to_table(entry: &Descriptor) -> &[Descriptor; 512] {
    unsafe {
        &*guest_physical_to_host_virt(entry.output_address().0 as u64).as_ptr::<[Descriptor; 512]>()
    }
}

fn guest_page_fault(device: &ModelDevice) {
    // get EL
    let el: u8 = *device.get_register_mut("PSTATE_EL");
    assert_eq!(el, 1);

    // get VBAR_ELx
    let vbar_el1: u64 = *device.get_register_mut("VBAR_EL1");
    panic!("{vbar_el1:x}");

    // get page fault handler
    // set PC and execute until we hit an eret
}

fn take_arm_exception(
    device: &ModelDevice,
    target_el: u8,
    typ: u8,
    syndrome: u32,
    vaddr: u64,
    retaddr: u64,
    mut voff: u64,
) {
    let spsr = get_psr_from_pstate(device);
    let current_el = *device.get_register_mut::<u8>("PSTATE_EL");

    if target_el > current_el {
        voff += 0x400;
    } else if *device.get_register_mut::<u8>("PSTATE_SP") == 1 {
        voff += 0x200;
    }

    // Update the execution level
    *device.get_register_mut::<u8>("PSTATE_EL") = target_el;

    // Update spsel
    *device.get_register_mut::<u8>("PSTATE_SP") = 1;

    if target_el == 1 {
        *device.get_register_mut::<u32>("SPSR_EL1") = spsr;
        *device.get_register_mut::<u64>("ELR_EL1") = retaddr;

        // If it's NOT an IRQ...
        if typ != 255 {
            let ec = get_exception_class(current_el, target_el, typ);
            *device.get_register_mut::<u32>("ESR_EL1") =
                (ec << 26) | (1 << 25) | (syndrome & 0x1ffffff);

            if typ == 1 || typ == 4 {
                *device.get_register_mut::<u64>("FAR_EL1") = vaddr;
            }
        }
    } else {
        panic!("trap");
    }

    *device.get_register_mut::<u8>("PSTATE_D") = 1;
    *device.get_register_mut::<u8>("PSTATE_A") = 1;
    *device.get_register_mut::<u8>("PSTATE_I") = 1;
    *device.get_register_mut::<u8>("PSTATE_F") = 1;

    // let vbar = read_register(VBAR);
    let vbar: u64 = match target_el {
        1 => *device.get_register_mut("VBAR_EL1"),
        2 => *device.get_register_mut("VBAR_EL2"),
        3 => *device.get_register_mut("VBAR_EL3"),
        _ => panic!(),
    };
    *device.get_register_mut::<u64>("_PC") = vbar + voff;
    return;
}

fn get_psr_from_pstate(device: &ModelDevice) -> u32 {
    let n = *device.get_register_mut::<u8>("PSTATE_N") as u32;
    let z = *device.get_register_mut::<u8>("PSTATE_Z") as u32;
    let c = *device.get_register_mut::<u8>("PSTATE_C") as u32;
    let v = *device.get_register_mut::<u8>("PSTATE_V") as u32;
    let d = *device.get_register_mut::<u8>("PSTATE_D") as u32;
    let a = *device.get_register_mut::<u8>("PSTATE_A") as u32;
    let i = *device.get_register_mut::<u8>("PSTATE_I") as u32;
    let f = *device.get_register_mut::<u8>("PSTATE_F") as u32;
    let el = *device.get_register_mut::<u8>("PSTATE_EL") as u32;
    let sp = *device.get_register_mut::<u8>("PSTATE_SP") as u32;

    n << 31 | z << 30 | c << 29 | v << 28 | d << 9 | a << 8 | i << 7 | f << 6 | el << 2 | sp
}

fn get_exception_class(current_el: u8, target_el: u8, typ: u8) -> u32 {
    match typ {
        0 => {
            // Software Breakpoint
            0x38 + 4
        }
        1 => {
            if target_el == current_el {
                0x25
            } else {
                0x24
            }
        }

        2 => {
            // Undefined Fault
            0
        }
        3 => {
            // Supervisor Call
            0x11 + 4
        }
        4 => {
            // Instruction Abort
            if target_el == current_el { 0x21 } else { 0x20 }
        }
        5 => {
            // FPAccessTrap
            0x07
        }
        6 => {
            // Single Step
            0x32
        }
        _ => {
            panic!("trap");
        }
    }
}
