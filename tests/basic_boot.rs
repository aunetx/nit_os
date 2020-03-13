#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nit_os::architecture::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nit_os::{println, println_color, serial_print, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nit_os::architecture::testing::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    println_color!(red "Hello World!");
    println_color!(green "Hello World!");
    serial_println!("[ok]");
}
