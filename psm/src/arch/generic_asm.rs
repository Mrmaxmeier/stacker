use crate::StackDirection;

// NB: this could be nicer across multiple blocks but we cannot do it because of
// https://github.com/rust-lang/rust/issues/65847
extern_item! { {
    #![cfg_attr(asm, link(name="psm_s"))]

    #[cfg(asm)]
    fn rust_psm_stack_direction() -> u8;
    #[cfg(asm)]
    fn rust_psm_stack_pointer() -> *mut u8;

    #[cfg(all(switchable_stack, not(target_os = "windows")))]
    #[link_name="rust_psm_replace_stack"]
    fn _rust_psm_replace_stack(
        data: usize,
        callback: extern_item!(unsafe fn(usize) -> !),
        sp: *mut u8
    ) -> !;
    #[cfg(all(switchable_stack, not(target_os = "windows")))]
    #[link_name="rust_psm_on_stack"]
    fn _rust_psm_on_stack(
        data: usize,
        return_ptr: usize,
        callback: extern_item!(unsafe fn(usize, usize)),
        sp: *mut u8,
    );
    #[cfg(all(switchable_stack, target_os = "windows"))]
    fn rust_psm_replace_stack(
        data: usize,
        callback: extern_item!(unsafe fn(usize) -> !),
        sp: *mut u8,
        stack_base: *mut u8
    ) -> !;
    #[cfg(all(switchable_stack, target_os = "windows"))]
    fn rust_psm_on_stack(
        data: usize,
        return_ptr: usize,
        callback: extern_item!(unsafe fn(usize, usize)),
        sp: *mut u8,
        stack_base: *mut u8
    );
} }

#[cfg(all(switchable_stack, not(target_os = "windows")))]
#[inline(always)]
unsafe fn rust_psm_replace_stack(
    data: usize,
    callback: extern_item!(unsafe fn(usize) -> !),
    sp: *mut u8,
    _: *mut u8,
) -> ! {
    _rust_psm_replace_stack(data, callback, sp)
}

#[cfg(all(switchable_stack, not(target_os = "windows")))]
#[inline(always)]
unsafe fn rust_psm_on_stack(
    data: usize,
    return_ptr: usize,
    callback: extern_item!(unsafe fn(usize, usize)),
    sp: *mut u8,
    _: *mut u8,
) {
    _rust_psm_on_stack(data, return_ptr, callback, sp)
}

pub(crate) fn stack_direction() -> StackDirection {
    const ASC: u8 = StackDirection::Ascending as u8;
    const DSC: u8 = StackDirection::Descending as u8;
    unsafe {
        match rust_psm_stack_direction() {
            ASC => StackDirection::Ascending,
            DSC => StackDirection::Descending,
            _ => ::core::hint::unreachable_unchecked(),
        }
    }
}

pub(crate) fn stack_pointer() -> *mut u8 {
    unsafe { rust_psm_stack_pointer() }
}

#[cfg(switchable_stack)]
extern_item! { pub(crate) unsafe fn on_stack(
    data: usize,
    return_ptr: usize,
    callback: extern_item! { unsafe fn(usize, usize) },
    sp: *mut u8,
    base: *mut u8,
) {
    rust_psm_on_stack(data, return_ptr, callback, sp, base)
} }

#[cfg(switchable_stack)]
extern_item! { pub(crate) unsafe fn replace_stack(
    data: usize,
    callback: extern_item! { unsafe fn(usize) -> ! },
    sp: *mut u8,
    base: *mut u8,
) -> ! {
    rust_psm_replace_stack(data, callback, sp, base)
} }
