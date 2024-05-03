use {
    cargo_metadata::{diagnostic::DiagnosticLevel, Artifact, Message},
    clap::Parser,
    itertools::Itertools,
    std::{
        fs::File,
        io::{BufReader, Write},
        path::{Path, PathBuf},
        process::{Command, Stdio},
    },
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Enable QEMU GDB server
    #[arg(long)]
    gdb: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Release profile build of kernel
    #[arg(short, long)]
    release: bool,

    /// Build only, do not start brig
    #[arg(long)]
    build_only: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    // create TAR file containing guest kernel, plugins, and configuration
    let guest_tar = build_guest_tar("./guest_data", cli.verbose, cli.release);

    // create an UEFI disk image of kernel
    let uefi_path = {
        let kernel_path = build_kernel("../brig", cli.verbose, cli.release);

        let uefi_path = kernel_path.parent().unwrap().join("uefi.img");
        bootloader::UefiBoot::new(&kernel_path)
            .create_disk_image(&uefi_path)
            .unwrap();

        println!("built UEFI image @ {uefi_path:?}");

        uefi_path
    };

    if cli.build_only {
        return Ok(());
    }

    // start QEMU with UEFI disk image
    run_brig(&uefi_path, &guest_tar, cli.gdb);

    Ok(())
}

fn build_cargo<P: AsRef<Path>>(project_path: P, args: Vec<&str>, verbose: bool) -> Vec<PathBuf> {
    let project_path = project_path.as_ref();
    println!("building {}...", project_path.to_str().unwrap());

    let mut cmd = Command::new("cargo");
    cmd.arg("build");

    args.iter().for_each(|arg| {
        cmd.arg(arg);
    });

    let mut cmd = cmd
        .arg("--message-format=json")
        .current_dir(project_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);

    let artifacts = Message::parse_stream(stdout_reader)
        .map(Result::unwrap)
        .map(|msg| {
            if verbose {
                if let Message::CompilerMessage(ref msg) = msg {
                    if let DiagnosticLevel::Error
                    | DiagnosticLevel::Ice
                    | DiagnosticLevel::FailureNote = msg.message.level
                    {
                        println!("{msg}");
                    }
                }
            }

            msg
        })
        .filter_map(|msg| {
            if let Message::CompilerArtifact(a) = msg {
                if let Some(path) = a.executable {
                    Some(path)
                } else {
                    if let Ok(x) = a.target.kind.into_iter().exactly_one() {
                        if x == "cdylib" {
                            Some(a.filenames.iter().exactly_one().unwrap().clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            } else {
                None
            }
        })
        .map(|a| a.canonicalize().unwrap())
        .collect::<Vec<_>>();

    assert!(cmd.wait().unwrap().success());

    println!("build complete.");

    artifacts
}

fn build_plugins<P: AsRef<Path>>(path: P, verbose: bool, release: bool) -> Vec<PathBuf> {
    println!("building plugins...");

    let mut args = Vec::new();
    if release {
        args.push("--release");
    }

    build_cargo(path, args, verbose)
}

fn build_guest_tar<P: AsRef<Path>>(guest_data_path: P, verbose: bool, release: bool) -> PathBuf {
    let guest_data_path = guest_data_path.as_ref();

    // build plugins
    let plugin_artifacts = build_plugins("../plugins", verbose, release);

    let target_dir = plugin_artifacts[0]
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // build device tree blob

    // dtc -I dts -O dtb -o $@ $<
    let dtb_path = target_dir.join("platform.dtb");
    let output = Command::new("dtc")
        .args([
            "-I",
            "dts",
            "-O",
            "dtb",
            "-o",
            dtb_path.to_str().unwrap(),
            guest_data_path.join("platform.dts").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    if !output.status.success() {
        panic!(
            "failed to create DTB:\n{}\n{}",
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()
        )
    }

    // // make rootfs.ext2
    // const ROOTFS_SIZE: usize = 512 * 1024 * 1024;
    // let volume = File::create(target_dir.join("rootfs.ext2")).unwrap();
    // volume.set_len(ROOTFS_SIZE);
    // assert!(Command::new("mkfs.ext2")
    //     .arg(volume.path(),)
    //     .output()
    //     .unwrap()
    //     .status
    //     .success());

    let tar_path = {
        let tar_path = target_dir.join("guest.tar");

        let mut tar = tar::Builder::new(File::create(&tar_path).unwrap());
        tar.append_file(
            "config.json",
            &mut File::open(guest_data_path.join("config.json")).unwrap(),
        )
        .unwrap();
        tar.append_file(
            "kernel",
            &mut File::open(guest_data_path.join("kernel")).unwrap(),
        )
        .unwrap();
        tar.append_file(
            "platform.dtb",
            &mut File::open(target_dir.join("platform.dtb")).unwrap(),
        )
        .unwrap();

        for path in plugin_artifacts {
            tar.append_file(
                PathBuf::from("plugins").join(path.file_name().unwrap()),
                &mut File::open(&path).unwrap(),
            )
            .unwrap();
        }

        tar.finish().unwrap();

        tar_path
    };

    tar_path
}

fn build_kernel<P: AsRef<Path>>(path: P, verbose: bool, release: bool) -> PathBuf {
    println!("building kernel...");

    let mut args = Vec::new();
    if release {
        args.push("--release");
    }

    let kernel_path = build_cargo(path, args, verbose)
        .into_iter()
        // get compiler artifact
        // todo: check this is an executable?
        .exactly_one()
        .map_err(|rest| {
            format!(
                "did not get exactly one matching compiler artifact: {:?}",
                rest.collect::<Vec<_>>()
            )
        })
        .unwrap();

    println!("built kernel @ {kernel_path:?}");

    kernel_path
}

fn run_brig(uefi_path: &Path, guest_tar_path: &Path, gdb: bool) {
    println!("starting QEMU");
    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive")
        .arg(format!("format=raw,file={}", uefi_path.to_str().unwrap()));
    cmd.arg("-nographic");
    cmd.arg("-enable-kvm");
    cmd.arg("-no-reboot");

    if gdb {
        cmd.arg("-gdb");
        cmd.arg("tcp::1234");
        cmd.arg("-S"); //  freeze CPU at startup
    }

    cmd.arg("-m");
    cmd.arg("8g");
    cmd.arg("-device");
    cmd.arg("virtio-blk-pci,drive=drive0,id=virtblk0,num-queues=4");
    cmd.arg("-drive");
    cmd.arg(format!(
        "file={},if=none,format=raw,id=drive0",
        guest_tar_path.display()
    ));
    cmd.arg("-device");
    cmd.arg("virtio-blk-pci,drive=drive1,id=virtblk1,num-queues=4");
    cmd.arg("-drive");
    cmd.arg("file=../../brig-programs/rootfs.ext2,if=none,format=raw,id=drive1");
    cmd.arg("-M");
    cmd.arg("q35");
    cmd.arg("-cpu");
    cmd.arg("host");

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
