//! This module enables convenient support for `QEMU`.
//!

// external crates
use x86_64::instructions::port::Port;

/// Defines the QEMU exit codes to be used when exiting with the
/// `isa-debug-exit` argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exit QEMU via the `isa-debug-exit` port, with the given exit code.
pub fn exit(exit_code: QemuExitCode) -> ! {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    loop {}
}
