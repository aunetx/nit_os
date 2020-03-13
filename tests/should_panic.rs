#![no_std]
#![no_main]

use core::panic::PanicInfo;
use nit_os::{
    architecture::qemu::{exit, QemuExitCode},
    serial_print, serial_println,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit(QemuExitCode::Failed);
}

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit(QemuExitCode::Success);
}
