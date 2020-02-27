use super::{exceptions, gdt, hardware};
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

// ! ------------- idt -------------

lazy_static! {
    /// The `Interrupt Descriptor Table`.
    ///
    /// It is used to store CPU exceptions (32 first bits) and CPU interrupts (next bits).
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

/// Init the `Interrupt Descriptor Table`.
pub fn init() {
    IDT.load();
}
