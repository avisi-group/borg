use {
    crate::{arch::x86::memory::guest_physical_to_host_virt, dbt::models::ModelDevice},
    aarch64_paging::paging::Descriptor,
};

// returns guest physical address
pub fn guest_translate(device: &ModelDevice, guest_virtual_address: u64) -> Option<u64> {
    let tcr_el1 = *device.get_register_mut::<u64>("TCR_EL1_bits");
    log::warn!("tcr_el1: {tcr_el1:x}");

    let translation_table_base_guest_phys = match guest_virtual_address {
        ..0x1000000000000 => *device.get_register_mut::<u64>("_TTBR0_EL1_bits"),
        0xffff000000000000.. => *device.get_register_mut::<u64>("_TTBR1_EL1_bits"),
        addr => todo!("fault at {addr:x}"),
    };

    let ttbgp_masked = translation_table_base_guest_phys & !0xffff000000000fff;

    log::warn!("guest_virtual_address: {guest_virtual_address:x?}");
    log::warn!("translation_table_base_guest_phys: {translation_table_base_guest_phys:x?}");
    log::warn!("ttbgp_masked: {ttbgp_masked:x?}");

    let translation_table_base = guest_physical_to_host_virt(ttbgp_masked);
    log::warn!("translation_table_base: {translation_table_base:x?}");
    let table = unsafe { &*(translation_table_base.as_ptr::<[Descriptor; 512]>()) };

    log::warn!("table: {table:x?}");

    translate_l1(table, guest_virtual_address)
}

fn translate_l0(table: &[Descriptor; 512], guest_virtual_address: u64) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 39) & 0x1ff) as usize;

    log::warn!("entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::warn!("entry: {entry:x?}");

    if entry.is_table_or_page() {
        translate_l1(entry_to_table(&entry), guest_virtual_address)
    } else {
        panic!("entry was not table or page")
    }
}

fn translate_l1(table: &[Descriptor; 512], guest_virtual_address: u64) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 30) & 0x1ff) as usize;
    log::warn!("entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::warn!("entry: {entry:x?}");

    if !entry.is_valid() {
        panic!("invalid")
    }

    if entry.is_table_or_page() {
        translate_l2(entry_to_table(&entry), guest_virtual_address)
    } else {
        todo!("block")
    }
}

fn translate_l2(table: &[Descriptor; 512], guest_virtual_address: u64) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 21) & 0x1ff) as usize;
    log::warn!("entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::warn!("entry: {entry:x?}");

    if !entry.is_valid() {
        panic!("invalid")
    }

    if entry.is_table_or_page() {
        translate_l3(entry_to_table(&entry), guest_virtual_address)
    } else {
        Some(entry.output_address().0 as u64)
    }
}

fn translate_l3(table: &[Descriptor; 512], guest_virtual_address: u64) -> Option<u64> {
    let entry_idx = ((guest_virtual_address >> 21) & 0x1ff) as usize;
    log::warn!("entry_idx: {entry_idx:x?}");
    let entry = table[entry_idx];
    log::warn!("entry: {entry:x?}");
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
