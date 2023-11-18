use core::arch::asm;
use core::ffi::CStr;
use core::num::NonZeroU32;

pub fn sys_dynlib_load_prx<N>(name: N) -> Result<u32, SysErr>
where
    N: AsRef<CStr>,
{
    let name = name.as_ref().as_ptr();
    let mut id = 0u32;
    let mut cf = 0u8;
    let mut rax = 0u64;
    let mut rdx = 0u64;

    unsafe {
        asm!(
            "xor rsi, rsi",
            "xor r10, r10",
            "mov rax, 594",
            "syscall",
            "setc [r12]",
            "mov [r13], rax",
            "mov [r14], rdx",
            in("rdi") name,
            in("rdx") &mut id,
            in("r12") &mut cf,
            in("r13") &mut rax,
            in("r14") &mut rdx,
            clobber_abi("C")
        )
    };

    if cf != 0 {
        Err(SysErr(NonZeroU32::new(rax.try_into().unwrap()).unwrap()))
    } else {
        Ok(id)
    }
}

pub struct SysErr(NonZeroU32);
