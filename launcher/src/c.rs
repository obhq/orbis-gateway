use crate::kernel::sceKernelLoadStartModule;
use crate::syscalls::sys_dynlib_dlsym;
use core::ffi::CStr;
use core::ptr::null;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// # Panics
/// If called a second time.
pub fn init_c() {
    // Check if initialized.
    if LOADED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        panic!("init_c can be called only once");
    }

    // Get module handle.
    let handle = unsafe {
        sceKernelLoadStartModule(b"libSceLibcInternal.sprx\0".as_ptr().cast(), 0, null(), 0)
            .unwrap()
    };

    // Resolve functions.
    for (name, ptr) in FUNCTION_TABLE {
        ptr.store(
            sys_dynlib_dlsym(handle.get(), name).unwrap(),
            Ordering::Relaxed,
        );
    }
}

static LOADED: AtomicBool = AtomicBool::new(false);
static MALLOC: AtomicUsize = AtomicUsize::new(0);
static FREE: AtomicUsize = AtomicUsize::new(0);
static FUNCTION_TABLE: [(&'static CStr, &'static AtomicUsize); 2] = [
    (
        unsafe { CStr::from_bytes_with_nul_unchecked(b"free\0") },
        &FREE,
    ),
    (
        unsafe { CStr::from_bytes_with_nul_unchecked(b"malloc\0") },
        &MALLOC,
    ),
];
