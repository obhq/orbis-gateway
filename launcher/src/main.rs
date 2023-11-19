#![no_std]
#![no_main]

use crate::kernel::init_kernel;
use core::arch::global_asm;
use core::ffi::c_int;
use core::panic::PanicInfo;
use core::ptr::null;

mod kernel;
mod syscalls;

global_asm!(
    ".globl _start",
    ".section .text.entry",
    "_start:",
    "lea rsi, [rip]",
    "sub rsi, 7",
    "mov rdx, rsi",
    "add rdx, 0x20", // Dynamic section hard-coded in link.ld.
    "jmp main"
);

#[no_mangle]
pub extern "C" fn main(_: *const (), base: *const u8, mut dynamic: *const usize) -> c_int {
    // Relocate ourself. Do not call any non-const functions or reference any global variables until
    // relocation is completed.
    let mut relocs = null();
    let mut relsz = 0;
    let mut relent = 0;

    loop {
        let tag = unsafe { *dynamic };
        let val = unsafe { *dynamic.add(1) };

        match tag {
            0 => break,
            7 => relocs = unsafe { base.add(val) },
            8 => relsz = val,
            9 => relent = val,
            _ => {}
        }

        dynamic = unsafe { dynamic.add(2) };
    }

    while relsz > 0 {
        let info: u64 = unsafe { core::ptr::read(relocs.add(8) as _) };

        match info & 0xffffffff {
            0 => break,
            8 => unsafe {
                let offset: usize = core::ptr::read(relocs as _);
                let addend: isize = core::ptr::read(relocs.add(16) as _);
                let addr = base.add(offset) as *mut usize;

                *addr = (base as usize).wrapping_add_signed(addend);
            },
            _ => {}
        }

        relocs = unsafe { relocs.add(relent) };
        relsz -= relent;
    }

    // Initialize libraries.
    init_kernel();

    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
