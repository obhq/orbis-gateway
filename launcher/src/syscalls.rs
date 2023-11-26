use core::arch::asm;
use core::ffi::{c_int, CStr};
use core::num::NonZeroI32;

pub fn write(fd: c_int, buf: *const u8, nbytes: usize) -> Result<usize, SysErr> {
    unsafe { syscall(4, fd, buf, nbytes, 0).map(|v| v.0) }
}

pub fn sys_dynlib_dlsym<N>(module: u32, name: N) -> Result<usize, SysErr>
where
    N: AsRef<CStr>,
{
    let mut addr = 0usize;

    unsafe {
        syscall(
            591,
            module,
            name.as_ref().as_ptr(),
            &mut addr as *mut usize,
            0,
        )?
    };

    Ok(addr)
}

pub fn sys_dynlib_load_prx<N>(name: N) -> Result<u32, SysErr>
where
    N: AsRef<CStr>,
{
    let mut id = 0u32;
    unsafe { syscall(594, name.as_ref().as_ptr(), 0, &mut id as *mut u32, 0)? };
    Ok(id)
}

unsafe fn syscall<A1, A2, A3, A4>(
    id: u32,
    arg1: A1,
    arg2: A2,
    arg3: A3,
    arg4: A4,
) -> Result<(usize, usize), SysErr>
where
    A1: Into<SysArg>,
    A2: Into<SysArg>,
    A3: Into<SysArg>,
    A4: Into<SysArg>,
{
    let mut cf = 0u8;
    let mut rax = 0usize;
    let mut rdx = 0usize;

    asm!(
        "syscall",
        "setc [r12]",
        "mov [r13], rax",
        "mov [r14], rdx",
        in("rax") id,
        in("rdi") arg1.into().get(),
        in("rsi") arg2.into().get(),
        in("rdx") arg3.into().get(),
        in("r10") arg4.into().get(),
        in("r12") &mut cf,
        in("r13") &mut rax,
        in("r14") &mut rdx,
        clobber_abi("C")
    );

    if cf != 0 {
        Err(SysErr(NonZeroI32::new(rax.try_into().unwrap()).unwrap()))
    } else {
        Ok((rax, rdx))
    }
}

#[derive(Debug)]
pub struct SysErr(NonZeroI32);

impl SysErr {
    pub fn new(errno: NonZeroI32) -> Self {
        Self(errno)
    }
}

#[derive(Clone, Copy)]
struct SysArg(usize);

impl SysArg {
    fn get(self) -> usize {
        self.0
    }
}

impl From<usize> for SysArg {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl<T> From<*mut T> for SysArg {
    fn from(value: *mut T) -> Self {
        Self(value as _)
    }
}

impl<T> From<*const T> for SysArg {
    fn from(value: *const T) -> Self {
        Self(value as usize)
    }
}

impl From<u32> for SysArg {
    fn from(value: u32) -> Self {
        Self(value as usize)
    }
}

impl From<i32> for SysArg {
    fn from(value: i32) -> Self {
        Self(value as usize)
    }
}
