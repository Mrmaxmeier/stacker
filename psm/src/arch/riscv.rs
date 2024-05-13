pub(crate) fn stack_direction() -> crate::StackDirection {
    crate::StackDirection::Descending
}

pub(crate) fn stack_pointer() -> *mut u8 {
    let mut ret;
    unsafe {
        core::arch::asm! {
            "mv {ret}, sp",
            ret = lateout(reg) ret,
            options(preserves_flags, nomem),
        }
    }
    ret
}

pub(crate) unsafe extern "C" fn replace_stack(
    data: usize,
    callback: unsafe extern "C" fn(usize) -> !,
    sp: *mut u8,
    _: *mut u8,
) -> ! {
    core::arch::asm! {
        "mv sp, {new_sp}",
        "jr {callback}",
        new_sp = in(reg) sp,
        callback = in(reg) callback,
        in("x10") data,
        options(noreturn, nostack),
    }
}

#[cfg(target_arch = "riscv64")]
core::arch::global_asm! {
    ".balign 8",
    ".globl rust_psm_on_stack",
    ".hidden rust_psm_on_stack",
    ".type rust_psm_on_stack STT_FUNC",
    "rust_psm_on_stack:",
    ".cfi_startproc",
    "sd x1, -8(x13)",
    "sd x2, -16(x13)",
    "addi x2, x13, -16",
    ".cfi_def_cfa x2, 16",
    ".cfi_offset x1, -8",
    ".cfi_offset x2, -16",
    "jalr x1, x12, 0",
    "ld x1, 8(x2)",
    ".cfi_restore x1",
    "ld x2, 0(x2)",
    ".cfi_restore x2",
    "jr x1",
    ".cfi_endproc",
}

#[cfg(target_arch = "riscv32")]
core::arch::global_asm! {
    ".balign 8",
    ".globl rust_psm_on_stack",
    ".hidden rust_psm_on_stack",
    ".type rust_psm_on_stack STT_FUNC",
    "rust_psm_on_stack:",
    ".cfi_startproc",
    "sw x1, -12(x13)",
    "sw x2, -16(x13)",
    "addi x2, x13, -16",
    ".cfi_def_cfa x2, 16",
    ".cfi_offset x1, -12",
    ".cfi_offset x2, -16",
    "jalr x1, x12, 0",
    "lw x1, 4(x2)",
    ".cfi_restore x1",
    "lw x2, 0(x2)",
    ".cfi_restore x2",
    "jr x1",
    ".cfi_endproc",
}

extern "C" {
    fn rust_psm_on_stack(
        data: usize,
        return_ptr: usize,
        callback: unsafe extern "C" fn(usize, usize),
        sp: *mut u8,
    );
}

pub(crate) unsafe fn on_stack(
    data: usize,
    return_ptr: usize,
    callback: unsafe extern "C" fn(usize, usize),
    sp: *mut u8,
    _: *mut u8,
) {
    rust_psm_on_stack(data, return_ptr, callback, sp)
}
