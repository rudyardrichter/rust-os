use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn clear_row(&mut self, row: usize) -> () {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    fn new_line(&mut self) -> () {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read());
            }
        }
        self.clear_row(VGA_BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn write_byte(&mut self, byte: u8) -> () {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }
                let row = VGA_BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) -> () {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output")
}

#[test_case]
fn test_println_many() {
    for i in 0..200 {
        println!("test_println_many output {}", i)
    }
}

#[test_case]
fn test_println_output() {
    let s = "Test string";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        // -2 because newline at the bottom of the screen after println
        let screen_char = WRITER.lock().buffer.chars[VGA_BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
