//! `amd64` architecture type.
//!
//! This is the default one, and should provide most implementations.
//!

// internal crate
use crate::interrupts::{gdt, idt, PICS};

// external crates
use x86_64::instructions;

/// Initialize architecture-specific parts of the kernel.
///
/// The default steps are :
/// - init GDT : `Global Descriptor Table`
/// - init IDT : `Interrupt Descriptor Table`
/// - init PICs chips : `Programmable Interrupt Controller`
/// - enable interrupts with asm instruction `sti`
pub fn init() {
    gdt::init();
    idt::init();
    unsafe { PICS.lock().initialize() };
    instructions::interrupts::enable();
}

/// Function halting the kernel : an endless loop catching interrupts.
pub fn halt_loop() -> ! {
    loop {
        instructions::hlt();
    }
}

/// Function "stopping" the kernel : an endless loop without interrupts.
pub fn stop_loop() -> ! {
    instructions::interrupts::disable();
    loop {
        instructions::hlt();
    }
}
