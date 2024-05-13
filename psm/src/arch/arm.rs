pub(crate) fn stack_direction() -> crate::StackDirection {
    crate::StackDirection::Descending
}

pub(crate) fn stack_pointer() -> *mut u8 {
    let mut ret;
    unsafe {
        core::arch::asm! {
            "mov {ret}, sp",
            ret = lateout(reg) ret,
            options(preserves_flags, nomem),
        }
    }
    ret
}

pub(crate) unsafe extern "aapcs" fn replace_stack(
    data: usize,
    callback: unsafe extern "aapcs" fn(usize) -> !,
    sp: *mut u8,
    _: *mut u8,
) -> ! {
    core::arch::asm! {
        "mov sp, {new_sp}",
        "bx {callback}",
        "udf #0",
        new_sp = in(reg) sp,
        callback = in(reg) callback,
        in("r0") data,
        options(noreturn, nostack),
    }
}

core::arch::global_asm! {
    ".balign 8",
    ".globl rust_psm_on_stack",
    ".hidden rust_psm_on_stack",
    ".type rust_psm_on_stack STT_FUNC",
    "rust_psm_on_stack:",
    ".fnstart",
    ".cfi_startproc",
    "push {{fp, lr}}",
    ".cfi_def_cfa_offset 8",
    ".cfi_offset lr, -4",
    ".cfi_offset fp, -8",
    "mov fp, sp",
    ".cfi_def_cfa_register fp",
    "mov sp, r3",
    "blx r2",
    "mov sp, fp",
    ".cfi_def_cfa_register sp",
    "pop {{fp, pc}}",
    ".cfi_endproc",
    ".fnend",
}

extern "aapcs" {
    fn rust_psm_on_stack(
        data: usize,
        return_ptr: usize,
        callback: unsafe extern "aapcs" fn(usize, usize),
        sp: *mut u8,
    );
}

pub(crate) unsafe fn on_stack(
    data: usize,
    return_ptr: usize,
    callback: unsafe extern "aapcs" fn(usize, usize),
    sp: *mut u8,
    _: *mut u8,
) {
    rust_psm_on_stack(data, return_ptr, callback, sp)
}
