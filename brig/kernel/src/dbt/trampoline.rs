use core::arch::global_asm;

global_asm!(
    r#"
        .global execute
    execute:
        push %rax
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

        mov %rsi, %rbp
        mov %rsp, %r14
        sub $0x1000, %rsp
        call *%rdi
        add $0x1000, %rsp

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
        pop %rax

        ret
   "#,
    options(att_syntax)
);

unsafe extern "C" {
    pub fn execute(code_ptr: *const u8, register_file: *mut u8);
}
