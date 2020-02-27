// internal functions used
use crate::gdt;

// external crates used
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

// submodules
mod exceptions;
mod hardware;

// submodules exports
pub use hardware::PICS;

// ! ------------- idt -------------

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // load exceptions handlers
        //breakpoint
        idt.breakpoint.set_handler_fn(exceptions::breakpoint_handler);
        //double_fault
        unsafe {
            idt.double_fault
                .set_handler_fn(exceptions::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        // load interrupts handlers
        //timer
        idt[hardware::InterruptIndex::Timer.as_usize()].set_handler_fn(hardware::timer_interrupt_handler);
        //keyboard
        idt[hardware::InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(hardware::keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// ! ------------- tests -------------

// internal functions used
#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_breakpoint_exception() {
    serial_print!("test_breakpoint_exception...");
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}
