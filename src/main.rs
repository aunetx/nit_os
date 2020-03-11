#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nit_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// the actual library
use nit_os::*;

// enable the builtin alloc crate
extern crate alloc;

// external crates used
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// permits to check the signature of the entry point
entry_point!(kernel_main);

/// The starting point of our kernel.
#[no_mangle]
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // ! ------------- initialisation -------------
    phase!(init(); "kernel init");
    println!("Everything seems to work!");

    // ! ------------- main -------------

    // ! ------------- tests -------------
    // define the entry of unit tests
    #[cfg(test)]
    test_main();

    // ! ------------- halting -------------
    // halt the kernel
    hlt_loop();
}

/// This function is called on panic.
// if not in test, print to vga buffer
#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println_color!(
        drivers::vga::Color::LightGray,
        drivers::vga::Color::Red,
        "[ kernel panic ]"
    );
    println_color!(red " {}", info);

    hlt_loop();
}

// if in unit test, print to serial and exit QEMU
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
