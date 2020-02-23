#[macro_use]
pub mod vga;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print_color!(vga::Color::LightGray, vga::Color::Red, "[ kernel panic ]");
    println_color!(red " {}", info);
    loop {}
}
