use crate::syscalls::{sys_dynlib_dlsym, sys_dynlib_load_prx, SysErr};
use core::ffi::{c_char, c_int, c_uint, c_void, CStr};
use core::mem::transmute;
use core::num::{NonZeroI32, NonZeroU32};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

pub fn errno() -> c_int {
    let f: Option<unsafe extern "C" fn() -> *mut c_int> =
        unsafe { transmute(ERRNO.load(Ordering::Relaxed)) };
    unsafe { *f.unwrap()() }
}

#[allow(non_snake_case)]
pub unsafe fn sceKernelLoadStartModule(
    name: *const c_char,
    argc: usize,
    argv: *const c_void,
    flags: c_uint,
) -> Result<NonZeroU32, SysErr> {
    let f: Option<
        unsafe extern "C" fn(*const c_char, usize, *const c_void, c_uint, c_int, c_int) -> c_int,
    > = transmute(LOAD_START_MODULE.load(Ordering::Relaxed));
    let r = f.unwrap()(name, argc, argv, flags, 0, 0);

    if r < 0 {
        Err(SysErr::new(NonZeroI32::new(errno()).unwrap()))
    } else {
        Ok(NonZeroU32::new(r.try_into().unwrap()).unwrap())
    }
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
