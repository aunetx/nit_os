//! This module is used for testing.
//!
//! Most parts of it are not compiled in normal compilation, but for the moment it
//! needs to be loaded in the kernel even if not in test.
//!

// ! ------------- unit tests -------------

// internal crate
use crate::{
    architecture::qemu::{exit, QemuExitCode},
    serial_println,
};

/// A function used during testing : run all the given tests.
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit(QemuExitCode::Success);
}

/// A function used during testing if test failed : print our error and exit.
pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit(QemuExitCode::Failed)
}

// ! ------------- integration tests -------------

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

// permits to check the signature of the entry point used in integration tests
#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point of integration tests.
#[cfg(test)]
#[no_mangle]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    super::init();
    crate::test_main();
    loop {}
}

/// Panic handler for integration tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}
