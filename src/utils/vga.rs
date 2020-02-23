use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    /// The actual VGA writer to be used by the kernel.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// ! ------------------ buffer ------------------

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const DEFAULT_FG_COLOR: Color = Color::Yellow;
const DEFAULT_BG_COLOR: Color = Color::Black;

/// A struct representing the entire VGA buffer.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A struct representing a character for the VGA buffer, containing its ASCII value and the `ColorCode` associated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// A struct representing colors for a character for the VGA buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Create a new `ColorCode` from a foreground and a background color.
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Enum representing a color (either foreground or background) for the VGA buffer.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// ! ------------------ writer ------------------

/// A writer for the VGA buffer.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Set color of the VGA writer.
    pub fn set_color(&mut self, fg_color: Color, bg_color: Color) {
        self.color_code = ColorCode::new(fg_color, bg_color);
    }

    /// Reset color of the VGA writer to the default ones.
    pub fn reset_color(&mut self) {
        self.color_code = ColorCode::new(DEFAULT_FG_COLOR, DEFAULT_BG_COLOR);
    }

    /// Write an entire ASCII string to the VGA buffer.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not printable in ASCII range : print '■'
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Write a single ASCII character to the VGA buffer.
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Write a new line to the VGA buffer.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clear the given row.
    fn clear_row(&mut self, row: usize) {
        // Overwrite every character in the row by a space
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(ScreenChar {
                ascii_character: b' ',
                color_code: self.color_code,
            });
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Print a line of text to the VGA buffer with a given color.
/// Defaults are : `red`, `green` and `blue`.
#[macro_export]
macro_rules! print_color {
    (red $($arg:tt)*) => (
        crate::vga::WRITER.lock().set_color(crate::vga::Color::Red, crate::vga::Color::Black);
        $crate::vga::_print(format_args!($($arg)*));
        crate::vga::WRITER.lock().reset_color();
    );
    (green $($arg:tt)*) => (
        crate::vga::WRITER.lock().set_color(crate::vga::Color::Green, crate::vga::Color::Black);
        $crate::vga::_print(format_args!($($arg)*));
        crate::vga::WRITER.lock().reset_color();
    );
    (blue $($arg:tt)*) => (
        crate::vga::WRITER.lock().set_color(crate::vga::Color::Blue, crate::vga::Color::Black);
        $crate::vga::_print(format_args!($($arg)*));
        crate::vga::WRITER.lock().reset_color();
    );
    ($fg:expr, $bg:expr, $($arg:tt)*) => (
        crate::vga::WRITER.lock().set_color($fg, $bg);
        $crate::vga::_print(format_args!($($arg)*));
        crate::vga::WRITER.lock().reset_color();
    );
}

#[macro_export]
macro_rules! println_color {
    ($tk:tt) => ($crate::print_color!($tk "\n"));
    ($fg:expr, $bg:expr) => ($crate::print_color!($fg, $bg, "\n"));
    ($tk:tt $($arg:tt)*) => (
        $crate::print_color!($tk $($arg)*);
        $crate::println!();
    );
    ($fg:expr, $bg:expr, $($arg:tt)*) => ($crate::print_color!($fg, $bg,  "{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
