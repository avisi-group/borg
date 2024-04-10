use {
    cargo_metadata::{Artifact, Message},
    itertools::Itertools,
    std::{
        io::BufReader,
        process::{Command, Stdio},
    },
};

fn main() {
    // build kernel
    // cargo b --message-format=json
    println!("building kernel");

    let mut cmd = Command::new("cargo")
        .args(["build", "--message-format=json"])
        .current_dir("../kernel")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);

    let kernel_path = Message::parse_stream(stdout_reader)
        .map(Result::unwrap)
        // get executable compiler artifacts
        .filter_map(|msg| {
            if let Message::CompilerArtifact(Artifact {
                executable: Some(path),
                ..
            }) = msg
            {
                Some(path)
            } else {
                None
            }
        })
        // get *one* executable compiler artifact path
        .exactly_one()
        .map_err(|rest| {
            format!(
                "did not get exactly one matching compiler artifact: {:?}",
                rest.collect::<Vec<_>>()
            )
        })
        .unwrap()
        .canonicalize()
        .unwrap();

    cmd.wait().unwrap();

    println!("built kernel @ {kernel_path:?}");

    // create an UEFI disk image
    let uefi_path = kernel_path.parent().unwrap().join("uefi.img");
    bootloader::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path)
        .unwrap();

    println!("built UEFI image @ {uefi_path:?}");

    // start QEMU with UEFI disk image
    println!("starting QEMU");
    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", uefi_path.to_str().unwrap()));
    cmd.arg("-nographic");
    cmd.arg("-enable-kvm");
    cmd.arg("-m");
    cmd.arg("8g");
    cmd.arg("-device");
    cmd.arg("virtio-blk-pci,drive=drive0,id=virtblk0,num-queues=4");
    cmd.arg("-drive");
    cmd.arg("file=../../brig-linux/brig-arm64-virt.tar,if=none,format=raw,id=drive0");
    cmd.arg("-device");
    cmd.arg("virtio-blk-pci,drive=drive1,id=virtblk1,num-queues=4");
    cmd.arg("-drive");
    cmd.arg("file=../../brig-linux/rootfs.ext2,if=none,format=raw,id=drive1");
    cmd.arg("-M");
    cmd.arg("q35");
    cmd.arg("-cpu");
    cmd.arg("host");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
