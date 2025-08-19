#![no_std]
#![no_main]

// core ------> cortex ------> HAL
use core::panic::PanicInfo;
use cortex_m::peripheral::SCB;
use cortex_m_rt::entry;
use stm32h7xx_hal as hal;
use hal::pac;
use hal::prelude::*;

// Rust requires a panic handler implementation
// for more info, read the docs.
#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    // reset the board
    // might have to change impl
    SCB::sys_reset();
}

// link address of kernel (where the kernel is placed)
const KERNEL_LINK_ADDR: u32 = 0x0802_0000;

// CPU freq_hz
const CPU_FREQ_HZ: u32 = 

fn syst_init(syst: &mut cortex_m::peripheral::SYST, sys_freq_hz: u32, interval_ms: u32) {
    const sys_calib: u32 = 0x3e8; // 1000
    let reload_val = (sys_freq_hz / (sys_calib / interval_ms)) -1; 

    syst.set_reload_value(reload_val);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();
    syst.set_clock_source(cortex_m::peripheral::SystClkSource::Core);
}

fn reallocate_vector_table(new_addr: u32) {
    // get pointer to SCB (System Control Block)
    let scb_ptr = SCB::PTR; 

    unsafe {
        // set the vtables to point to new addr
        (*scb_ptr).vtor.write(new_addr);
    }
}

// entry point of kernel
#[entry]
fn main() -> ! {
    // reallocate vector tables
    reallocate_vector_table(KERNEL_LINK_ADDR);

    // enable gloabal interrupts
    unsafe {
        cortex_m::interrupt::enable();
    } 

    // take ownership of cortex and device peripherals
    let cp = cortex_m::Peripherals::take()
        .unwrap();
    let dp = pac::Peripherals::take()
        .unwrap();

    // set up power
    let pwr = dp.PWR
        .constrain();
    let pwrcfg = pwr
        .freeze();

    // set up the clocks
    let rcc = dp.RCC
        .constrain();
    let ccdr = rcc
        .sys_ck(400.MHz())
        .freeze(pwrcfg, &dp.SYSCFG);

    syst_init(&mut cp.SYST, 50);

    loop {}
}
