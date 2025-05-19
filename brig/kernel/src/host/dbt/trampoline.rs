use {bitfields::bitfield, core::arch::asm};

pub const MAX_STACK_SIZE: usize = 2 * 1024 * 1024;

#[bitfield(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExecutionResult {
    need_tlb_invalidate: bool,
    interrupt_pending: bool,
    #[bits(30)]
    _reserved: u32,
}

impl ExecutionResult {
    pub fn as_u32(&self) -> u32 {
        self.into_bits()
    }
}

#[inline(never)] // only disabled to make debugging easier
pub fn trampoline(code_ptr: *const u8, register_file: *mut u8) -> ExecutionResult {
    let mut status: u32;

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
