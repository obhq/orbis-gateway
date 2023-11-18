use crate::syscalls::sys_dynlib_load_prx;
use core::ffi::CStr;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// # Panics
/// If called a second time.
pub fn init_kernel() {
    if LOADED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        panic!("init_kernel can be called only once");
    }

    // Try all variants.
    let variants = [
        unsafe { CStr::from_bytes_with_nul_unchecked(b"libkernel.sprx\0") },
        unsafe { CStr::from_bytes_with_nul_unchecked(b"libkernel_web.sprx\0") },
        unsafe { CStr::from_bytes_with_nul_unchecked(b"libkernel_sys.sprx\0") },
    ];

    for name in variants {
        if let Ok(v) = sys_dynlib_load_prx(name) {
            HANDLE.store(v, Ordering::Relaxed);
            return;
        }
    }

    panic!("cannot load libkernel.sprx");
}

static LOADED: AtomicBool = AtomicBool::new(false);
static HANDLE: AtomicU32 = AtomicU32::new(0);
