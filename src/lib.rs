#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// external crates used
use core::panic::PanicInfo;

// submodules exports
pub mod interrupts;
pub mod serial;
pub mod vga;

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

/// Entry point of integration tests.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
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
