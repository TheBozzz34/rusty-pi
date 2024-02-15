#![no_std]
#![no_main]


// 3F20_0008 fsel2 1<<3 turn pin 21 into output
// 3F20_001C gpio1_set 1<<21 turns pin 21 on
// 3F20_0028 gpio1_clear 1<<21 turns pin 21 off

use core::panic::PanicInfo;
use core::arch::asm;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

//GPIO addresses
const GPIO_BASE: u32 = 0x3F20_0000;
const GPIO_FSEL2: u32 = GPIO_BASE + 0x08;
const GPIO_SET: u32 = GPIO_BASE + 0x1C;
const GPIO_CLEAR: u32 = GPIO_BASE + 0x28;
const PIN_21: u32 = 1 << 21;

// Timer base address
const TIMER_BASE: u32 = 0x3F00_3000;

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


// Function to set a pin as an output
fn set_pin_output(pin: u32) {
    unsafe {
        let reg = (GPIO_FSEL2 as *mut u32).add((pin / 10) as usize);
        let shift = (pin % 10) * 3;
        let mask = 0b111 << shift;
        let value = 0b001 << shift;
        let current = core::ptr::read_volatile(reg);
        core::ptr::write_volatile(reg, (current & !mask) | value);
    }
} 

// Function to set a pin high
fn set_pin_high(pin: u32) {
    unsafe {
        core::ptr::write_volatile((GPIO_SET + ((pin / 32) * 4)) as *mut u32, 1 << (pin % 32));
    }
}

// Function to set a pin low
fn set_pin_low(pin: u32) {
    unsafe {
        core::ptr::write_volatile((GPIO_CLEAR + ((pin / 32) * 4)) as *mut u32, 1 << (pin % 32));
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Set pin 21 as an output
    set_pin_output(PIN_21);

    loop {
        // Turn pin 21 on and off every five seconds
        set_pin_high(PIN_21);
        delay_ms(5000); // Non blocking delay for five seconds
        set_pin_low(PIN_21);
        delay_ms(5000);
    }
}


#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop{}
}
