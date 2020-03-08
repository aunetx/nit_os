#![no_std]
#![cfg_attr(test, no_main)]
#![feature(const_fn)]
#![feature(custom_test_frameworks)]
#![feature(alloc_layout_extra)]
#![feature(const_in_array_repeat_expressions)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// enable the builtin alloc crate
extern crate alloc;

// external crates used
use core::panic::PanicInfo;

// submodules exports
pub mod drivers;
pub mod interrupts;
pub mod memory;

/// Initialize our kernel.
///
/// The default steps are :
/// - init GDT : `Global Descriptor Table`
/// - init IDT : `Interrupt Descriptor Table`
/// - init PICs chips : `Programmable Interrupt Controller`
/// - enable interrupts with asm instruction `sti`
pub fn init() {
    interrupts::gdt::init();
    interrupts::idt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

/// Function halting the kernel : an endless loop catching interrupts.
pub fn hlt_loop() -> ! {
    x86_64::instructions::interrupts::disable();
    loop {
        x86_64::instructions::hlt();
    }
}

/// Defines the QEMU exit codes to be used when exiting with the `isa-debug-exit` argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exit QEMU via the `isa-debug-exit` port, with the given exit code.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// ! ------------- testing -------------

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

// permits to check the signature of the entry point used in integration tests
#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point of integration tests.
#[cfg(test)]
#[no_mangle]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    loop {}
}

/// Panic handler for integration tests.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// A function used during testing : run all the given tests.
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

/// A function used during testing if test failed : print our error and exit.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
