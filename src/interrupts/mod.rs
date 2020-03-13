//! This module contains utils set interrupts handlers.
//!
//! It should be used in the first parts of kernel init.
//!

// submodules
mod exceptions;
mod hardware;

// public submodules
pub mod gdt;
pub mod idt;

// submodules exports
pub use hardware::PICS;

// ! ------------- tests -------------

// internal crate
#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
