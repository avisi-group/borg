use {
    arch::{decode_execute, ExecuteResult, State, Tracer, REG_CTR_EL0, REG_R0, REG_R1, REG_U_PC},
    clap::Parser,
    rustix::mm::{MapFlags, ProtFlags},
    std::{fmt::Debug, fs, path::PathBuf, ptr, time::Instant},
};

const GUEST_MEMORY_BASE: usize = 0x10_000;
const GUEST_MEMORY_SIZE: usize = 12 * 1024 * 1024 * 1024;
const KERNEL_LOAD_BIAS: usize = 0x8000_0000;

fn main() {
    let cli = Cli::parse();

    let image = fs::read(cli.path).unwrap();

    let header = unsafe { &*(image.as_ptr() as *const Arm64KernelHeader) };
    if header.magic == ARM64_MAGIC {
        assert_eq!(0, header.text_offset);
    }

    let mut state = State::init(GUEST_MEMORY_BASE);

    state.write_register::<u64>(REG_CTR_EL0, 0x0444c004);
    state.write_register(REG_U_PC, KERNEL_LOAD_BIAS);
    state.write_register::<u64>(REG_R0, 0x823a6040);
    state.write_register::<u64>(REG_R1, 0x823a6000);

    let mut instructions_retired = 0u64;

    let mut last_instrs = 0;
    let mut last_time = Instant::now();

    let mmap = unsafe {
        rustix::mm::mmap_anonymous(
            GUEST_MEMORY_BASE as *mut _,
            GUEST_MEMORY_SIZE,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::FIXED | MapFlags::PRIVATE,
        )
    }
    .unwrap();

    unsafe {
        ptr::copy(
            image.as_ptr(),
            (mmap as *mut u8).offset(KERNEL_LOAD_BIAS as isize),
            image.len(),
        )
    };

    loop {
        let pc = state.read_register::<u64>(REG_U_PC);

        let insn_data = unsafe { *(((mmap as *const u8).offset(pc as isize)) as *const u32) };

        let res = if cli.verbose {
            decode_execute(insn_data, &mut state, &PrintlnTracer)
        } else {
            decode_execute(insn_data, &mut state, &NoopTracer)
        };

        match res {
            ExecuteResult::Ok | ExecuteResult::EndOfBlock => {
                instructions_retired += 1;
            }

            ExecuteResult::UndefinedInstruction => {
                panic!("undefined instruction {:08x}", insn_data)
            }
        }

        if cli.bench && instructions_retired % (1024 * 1024) == 0 {
            let delta_instrs = instructions_retired - last_instrs;
            let delta_time = Instant::now() - last_time;
            println!(
                "{:.2}",
                (delta_instrs as f64 / delta_time.as_micros() as f64) * 1_000_000f64
            );
            last_instrs = instructions_retired;
            last_time = Instant::now();
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Enable tracing
    #[arg(short)]
    verbose: bool,
    /// Measure and print instructions / second at regular intervals
    #[arg(short)]
    bench: bool,
    /// Path to .text section to execute
    path: PathBuf,
}

struct NoopTracer;

impl Tracer for NoopTracer {
    fn begin(&self, _instruction: u32, _pc: u64) {}

    fn end(&self) {}

    fn read_register<T: Debug>(&self, _offset: isize, _value: T) {}

    fn write_register<T: Debug>(&self, _offset: isize, _value: T) {}
}

struct PrintlnTracer;

impl Tracer for PrintlnTracer {
    fn begin(&self, instruction: u32, pc: u64) {
        println!("[{instruction:x} @ {pc:x}] ");
    }

    fn end(&self) {
        println!();
    }

    fn read_register<T: Debug>(&self, offset: isize, value: T) {
        match arch::REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                println!("    R[{}] -> {value:x?}", arch::REGISTER_NAME_MAP[idx].1)
            }
            // we're accessing inside a register
            Err(idx) => {
                // get the register and print the offset from the base
                let (register_offset, name) = arch::REGISTER_NAME_MAP[idx - 1];
                println!("    R[{name}:{:x}] -> {value:x?}", offset - register_offset);
            }
        }
    }

    fn write_register<T: Debug>(&self, offset: isize, value: T) {
        match arch::REGISTER_NAME_MAP.binary_search_by(|(candidate, _)| candidate.cmp(&offset)) {
            Ok(idx) => {
                println!("    R[{}] <- {value:x?}", arch::REGISTER_NAME_MAP[idx].1)
            }
            Err(idx) => {
                let (register_offset, name) = arch::REGISTER_NAME_MAP[idx - 1];
                println!("    R[{name}:{:x}] <- {value:x?}", offset - register_offset);
            }
        }
    }
}

const ARM64_MAGIC: u32 = 0x644d5241;

#[derive(Debug)]
#[repr(C)]
struct Arm64KernelHeader {
    code0: u32,
    code1: u32,
    text_offset: u64,
    image_size: u64,
    flags: u64,
    res2: u64,
    res3: u64,
    res4: u64,
    magic: u32,
    res5: u32,
}
