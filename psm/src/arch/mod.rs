cfg_if::cfg_if! {
    if #[cfg(asm)] {
        #[path = "generic_asm.rs"]
        mod imp;
    } else if #[cfg(all(target_arch="x86", target_os="windows"))] {
        #[path = "x86_windows.rs"]
        mod imp;
    } else if #[cfg(target_arch="x86")] {
        #[path = "x86.rs"]
        mod imp;
    } else if #[cfg(all(target_arch="x86_64", target_os="windows"))] {
        #[path = "x86_64_windows.rs"]
        mod imp;
    } else if #[cfg(target_arch="x86_64")] {
        #[path = "x86_64.rs"]
        mod imp;
    } else if #[cfg(target_arch="arm")] {
        #[path = "arm.rs"]
        mod imp;
    } else if #[cfg(target_arch="aarch64")] {
        #[path = "aarch64.rs"]
        mod imp;
    } else if #[cfg(any(target_arch="riscv32", target_arch="riscv64"))] {
        #[path = "riscv.rs"]
        mod imp;
    }else {
        compile_error!("Target is not supported by the `psm` crate!");
    }
}

pub(crate) use self::imp::*;
