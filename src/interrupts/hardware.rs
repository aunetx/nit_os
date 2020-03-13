//! Defines functions to be called when an hardware interrupt occurs.
//!

// internal crate
use crate::{clear_screen, print};

// external crates
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

// ! ------------- interrupts structure -------------

/// Position of the first PIC.
pub const PIC_1_OFFSET: u8 = 32;
/// Position of the second PIC.
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// The PICS chips, protected by a mutual exclusion `spin::Mutex`.
pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Enum defining position of interrupt for the PICS chips.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    /// Convert the interrupt position to `u8`.
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Convert the interrupt position to `usize`.
    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

// ! ------------- interrupts handlers -------------

// TODO be able to "register" handler for interrupts (at compile-time?)

/// Interrupt handler for the hardware timer interruption.
///
/// By default, do nothing.
pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    }
}

/// Interrupt handler for the hardware keyboard interruption.
// TODO permit to register key interruptions
pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode('\u{8}') => clear_screen!(),
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8())
    }
}
