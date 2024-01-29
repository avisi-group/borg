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
    cmd.arg("8g");
    cmd.arg("-device");
    cmd.arg("virtio-blk-pci,drive=drive0,id=virtblk0,num-queues=4");
    cmd.arg("-drive");
    cmd.arg("file=../brig-linux/brig-arm64-virt.tar,if=none,format=raw,id=drive0");
    cmd.arg("-device");
    cmd.arg("virtio-blk-pci,drive=drive1,id=virtblk1,num-queues=4");
    cmd.arg("-drive");
    cmd.arg("file=../brig-linux/rootfs.ext2,if=none,format=raw,id=drive1");
    cmd.arg("-M");
    cmd.arg("q35");
    cmd.arg("-cpu");
    cmd.arg("host");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
