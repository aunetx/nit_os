#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nit_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nit_os::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println_color!(red "Hello World!");
    println_color!(green "Hello World!");
    println!("Hello World!");

    nit_os::init();

    #[cfg(test)]
    test_main();

    hlt_loop();
}

/// This function is called on panic.
// if not in test, print to vga buffer
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println_color!(vga::Color::LightGray, vga::Color::Red, "[ kernel panic ]");
    println_color!(red " {}", info);

    hlt_loop();
}

// if in test, print to serial and exit QEMU
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nit_os::test_panic_handler(info)
}
