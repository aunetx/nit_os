#![no_std]
#![no_main]

mod utils;

use utils::*;

/// This function is the entry point of the kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("error of fejd");
    // end of program
    loop {}
}
