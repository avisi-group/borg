use core::arch::asm;

pub const MAX_STACK_SIZE: usize = 2 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum ExecutionResult {
    Ok = 0,
    NeedTLBInvalidate,
    InterruptPending,
}

impl From<u64> for ExecutionResult {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Ok,
            1 => Self::NeedTLBInvalidate,
            2 => Self::InterruptPending,
            _ => panic!("unknown execution result value: {value:x}"),
        }
    }
}

impl ExecutionResult {
    pub fn as_u64(&self) -> u64 {
        *self as u64
    }
}

#[inline(never)] // only disabled to make debugging easier
pub fn trampoline(code_ptr: *const u8, register_file: *mut u8) -> ExecutionResult {
    let mut status: u64;

    unsafe {
        asm!(
            "
                push %rcx
                push %rdx
                push %rbx
                push %rbp
                push %rsi
                push %rdi
                push %r8
                push %r9
                push %r10
                push %r11
                push %r12
                push %r13
                push %r14
                push %r15

                mov {register_file}, %rbp
                mov %rsp, %r14
                sub ${max_stack_size}, %rsp
                call *{code_ptr}
                add ${max_stack_size}, %rsp

                pop %r15
                pop %r14
                pop %r13
                pop %r12
                pop %r11
                pop %r10
                pop %r9
                pop %r8
                pop %rdi
                pop %rsi
                pop %rbp
                pop %rbx
                pop %rdx
                pop %rcx
            ",
            options(att_syntax),
            max_stack_size = const MAX_STACK_SIZE,
            register_file = in(reg) register_file,
            code_ptr = in(reg) code_ptr,
            out("rax") status,
        )
    };

    ExecutionResult::from(status)
}
