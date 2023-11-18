#![no_std]
#![no_main]

use crate::kernel::init_kernel;
use core::arch::global_asm;
use core::ffi::c_int;
use core::panic::PanicInfo;
use core::ptr::{null, read_unaligned, write_unaligned};

mod kernel;
mod syscalls;

global_asm!(
    ".globl _start",
    ".section .text.entry",
    "_start:",
    "lea rsi, [rip]",
    "sub rsi, 7",
    "mov rdx, rsi",
    "add rdx, 0x4000", // Dynamic section hard-coded in link.ld.
    "jmp main"
);

#[no_mangle]
pub extern "C" fn main(_: *const (), base: *const u8, mut dynamic: *const u8) -> c_int {
    // Relocate ourself. Do not call any non-const functions or reference any global variables until
    // relocation is completed.
    let mut relocs = null();
    let mut relsz = 0;
    let mut relent = 0;

    loop {
        let tag: u64 = unsafe { read_unaligned(dynamic as _) };
        let val: usize = unsafe { read_unaligned(dynamic.add(8) as _) };

        match tag {
            0 => break,
            7 => relocs = unsafe { base.add(val) },
            8 => relsz = val,
            9 => relent = val,
            _ => {}
        }

        dynamic = unsafe { dynamic.add(16) };
    }

    while relsz > 0 {
        let offset: usize = unsafe { read_unaligned(relocs as _) };
        let info: u64 = unsafe { read_unaligned(relocs.add(8) as _) };
        let addend: isize = unsafe { read_unaligned(relocs.add(16) as _) };

        match info & 0xffffffff {
            0 => break,
            8 => unsafe {
                let addr = base.add(offset) as *mut usize;
                let val = (base as usize).wrapping_add_signed(addend);
                write_unaligned(addr, val);
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
