use crate::syscalls::{sys_dynlib_dlsym, sys_dynlib_load_prx};
use core::ffi::{c_int, CStr};
use core::mem::transmute;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub fn errno() -> c_int {
    let f: Option<unsafe extern "C" fn() -> *mut c_int> =
        unsafe { transmute(ERRNO.load(Ordering::Relaxed)) };
    unsafe { *f.unwrap()() }
}

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
        if let Ok(md) = sys_dynlib_load_prx(name) {
            for (name, ptr) in FUNCTION_TABLE {
                ptr.store(sys_dynlib_dlsym(md, name).unwrap(), Ordering::Relaxed);
            }
            return;
        }
    }

    panic!("cannot load libkernel.sprx");
}

static LOADED: AtomicBool = AtomicBool::new(false);
static ERRNO: AtomicUsize = AtomicUsize::new(0);
static LOAD_START_MODULE: AtomicUsize = AtomicUsize::new(0);
static FUNCTION_TABLE: [(&'static CStr, &'static AtomicUsize); 2] = [
    (
        unsafe { CStr::from_bytes_with_nul_unchecked(b"__error\0") },
        &ERRNO,
    ),
    (
        unsafe { CStr::from_bytes_with_nul_unchecked(b"sceKernelLoadStartModule\0") },
        &LOAD_START_MODULE,
    ),
];
