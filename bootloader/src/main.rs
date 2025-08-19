#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // handle the panic 
    cortex_m::peripheral::SCB::sys_reset(); 
}

const KERNEL_LINK_ADDR: u32 = 0x0802_0000; 

fn jump_to_kernel(kernel_start_addr: u32) -> ! {
    let sp = unsafe { *(kernel_start_addr as *const u32)};
    let rv = unsafe { *((kernel_start_addr + 4) as *const u32)};

    unsafe {
        asm! (
            "msr msp, {0}",
            "bx {1}",
            in(reg) sp,
            in(reg) rv,
            options(noreturn)
        );
    }
}

#[entry]
fn main() -> ! {
    jump_to_kernel(KERNEL_LINK_ADDR);
}
