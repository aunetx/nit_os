// internal functions used
use crate::println;

// external crates used
use x86_64::structures::idt::InterruptStackFrame;

// ! ------------- exceptions handlers -------------

/// Exception handler for the breakpoint exception.
///
/// By default, print to screen and continue.
pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Exception handler for the double fault exception.
///
/// By default, print to screen and trigger a kernel panic.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}\n", stack_frame);
}
