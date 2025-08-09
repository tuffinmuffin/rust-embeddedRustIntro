#![no_std]
#![no_main]
// cargo build --target thumbv7em-none-eabihf
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

mod startup_stm32f303;

static mut SCORES_GLOBAL: [i32; 5] = [1, 2, 3, 4, 5];
const _NUMBERS: [i32; 5] = [1, 2, 3, 4, 5]; // Example static array
static mut BUFFER: [u8; 1024] = [0; 1024]; // Example bss buffer

#[unsafe(no_mangle)]
fn main() {
    let mut _total_score = 0;

    unsafe {
        for score in SCORES_GLOBAL {
            _total_score += score;
        }
    }

    unsafe {
        BUFFER[0] = 100;
    }

    loop {

    }
}


#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    // Handle panic, e.g., log the panic message
    loop {}
}

