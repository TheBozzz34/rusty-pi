#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

// GPIO addresses
const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;


// Pin configuration
const PIN_21: u32 = 1 << 21;

// Timer base address
const SYSTEM_TIMER_BASE: u32 = 0x3F00_3000;

// Offsets for system timer registers
const SYSTEM_TIMER_CLO: u32 = SYSTEM_TIMER_BASE + 0x04;
const SYSTEM_TIMER_C0: u32 = SYSTEM_TIMER_BASE + 0x0C;

// Function to get the current system timer counter
fn get_system_timer() -> u64 {
    unsafe {
        let lo = core::ptr::read_volatile(SYSTEM_TIMER_CLO as *const u32) as u64;
        let hi = core::ptr::read_volatile(SYSTEM_TIMER_C0 as *const u32) as u64;
        (hi << 32) | lo
    }
}

// Delay function using the system timer
fn delay_ms(ms: u32) {
    let start = get_system_timer();
    let mut end = start + (ms as u64 * 1_000); // Convert ms to microseconds
    // If end overflows, adjust it
    if end < start {
        end = u64::max_value();
    }
    while get_system_timer() < end {}
}


struct GPIO;

impl GPIO {
    pub fn set_output(pin: u32) {

        let reg = pin/10;

        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Invalid pin number"),
        };

        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(register as *mut32); 
        }

        let mut mask: u32 = 0b111;

        let pinnum = pin%10;

        mask = mask << pinnum*3;

        val = val & !mask;

        val |= 1 << pinnum*3;

        unsafe {
            core::ptr::write_volatile(register as *   mut32, val); 
        }
    }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {

    loop {
    }
}


#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop{}
}
