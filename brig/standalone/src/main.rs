use {
    arch::{decode_execute, ExecuteResult, State, Tracer, REG_U_PC},
    rustix::mm::{MapFlags, ProtFlags},
    std::{env, fmt::Debug, fs, time::Instant},
};

struct NoopTracer;

impl Tracer for NoopTracer {
    fn begin(&self, _pc: u64) {}

    fn end(&self) {}

    fn read_register<T: Debug>(&self, _offset: usize, _value: T) {}

    fn write_register<T: Debug>(&self, _offset: usize, _value: T) {}
}

fn main() {
    unsafe {
        rustix::mm::mmap_anonymous(
            0x10_000 as *mut _,
            12 * 1024 * 1024 * 1024,
            ProtFlags::READ | ProtFlags::WRITE,
            MapFlags::FIXED | MapFlags::PRIVATE,
        )
    }
    .unwrap();

    let kernel_path = &env::args().collect::<Vec<_>>()[1];

    let kernel = fs::read(kernel_path).unwrap();

    // todo read kernel from TAR

    // read header from kernel
    let header = unsafe { &*(kernel.as_ptr() as *const Arm64KernelHeader) };
    assert_eq!(ARM64_MAGIC, header.magic);

    let mut state = State::init();

    let mut instructions_retired = 0u64;

    let mut last_instrs = 0;
    let mut last_time = Instant::now();

    loop {
        let pc = state.read_register::<u64>(REG_U_PC);
        let insn_data: u32 = unsafe { *(kernel.as_ptr().offset(pc as isize) as *const u32) };

        // println!("fetch @ {:x} = {:08x}", pc, insn_data);

        match decode_execute(insn_data, &mut state, &mut NoopTracer) {
            ExecuteResult::Ok | ExecuteResult::EndOfBlock => {
                instructions_retired += 1;
            }

            ExecuteResult::UndefinedInstruction => {
                panic!("undefined instruction {:08x}", insn_data)
            }
        }

        if instructions_retired % (1024 * 1024) == 0 {
            let delta_instrs = instructions_retired - last_instrs;
            let delta_time = Instant::now() - last_time;
            println!(
                "{}",
                (delta_instrs as f64 / delta_time.as_micros() as f64) * 1_000_000f64
            );
            last_instrs = instructions_retired;
            last_time = Instant::now();
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
