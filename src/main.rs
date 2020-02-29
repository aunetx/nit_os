#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nit_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// the actual library
use nit_os::*;

// external crates used
use core::panic::PanicInfo;

/// The starting point of our kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    phase!(init(), "kernel init");

    println!("Everything seems to work!");

    // define the entry of unit tests
    #[cfg(test)]
    test_main();

    // halt the kernel
    hlt_loop();
}

/// This function is called on panic.
// if not in test, print to vga buffer
#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println_color!(vga::Color::LightGray, vga::Color::Red, "[ kernel panic ]");
    println_color!(red " {}", info);

    hlt_loop();
}

// if in unit test, print to serial and exit QEMU
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
