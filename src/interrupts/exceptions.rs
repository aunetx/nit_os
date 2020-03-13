//! Defines functions to be called when an exception occurs.
//!

// internal crate
use crate::println;

// external crates
use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

// ! ------------- exceptions handlers -------------

/// Exception handler for the breakpoint exception.
///
/// By default, print to screen and continue.
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Exception handler for the page fault exception.
///
/// By default, print to screen and trigger a kernel panic.
pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!(
        "EXCEPTION: PAGE FAULT\nAccessed address: {:?}\nError code: {:?}\n{:#?}",
        Cr2::read(),
        error_code,
        stack_frame
    );
}

/// Exception handler for the double fault exception.
///
/// By default, print to screen and trigger a kernel panic.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
