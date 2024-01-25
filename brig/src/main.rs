fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");

    let mut cmd = std::process::Command::new("qemu-system-x86_64");

    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive")
        .arg(format!("format=raw,file={uefi_path}"));
    cmd.arg("-nographic");
    cmd.arg("-enable-kvm");
    cmd.arg("-m");
    cmd.arg("1g");
    cmd.arg("-device");
    cmd.arg("virtio-blk-pci,drive=drive0,id=virtblk0,num-queues=4");
    cmd.arg("-drive");
    cmd.arg("file=../rootfs.ext2,if=none,id=drive0");
    cmd.arg("-M");
    cmd.arg("q35");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
