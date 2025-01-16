use core::ops::RangeInclusive;

/// The default color to print the text (Light Cyan)
const DEFAULT_COLOR: u8 = 0xb;
/// The address to the VGA Buffer
const VGA_ADDR: usize = 0xb8000;
/// The height of the VGA buffer (number of rows)
const BUFFER_HEIGHT: usize = 25;
/// The width of the VGA buffer (number of columns)
const BUFFER_WIDTH: usize = 80;
/// The character to be printed when an invalid byte is sent
const INVALID_BYTE_CHAR: u8 = 0xfe;
/// The first valid character
const CHAR_VALID_RANGE_START: u8 = 0x20;
/// The last valid character
const CHAR_VALID_RANGE_END: u8 = 0x7e;
/// The new line character
const CHAR_NEW_LINE: u8 = b'\n';

/// All available colors to print
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

/// A struct to allow writing in the VGA buffer
///
/// Writes always in the last row and shifts text up as more rows are added
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Write a string in the VGA screen
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII byte or new line
                CHAR_VALID_RANGE_START..=CHAR_VALID_RANGE_END | CHAR_NEW_LINE => {
                    self.write_byte(byte)
                }
                // Invalid byte character
                _ => self.write_byte(INVALID_BYTE_CHAR),
            }
        }
    }

    /// Write a byte in the VGA screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            CHAR_NEW_LINE => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col] = ScreenChar::new(byte, color_code);

                self.column_position += 1;
            }
        }
    }

    /// Adds a new line to the current position
    fn new_line(&mut self) {
        todo!()
    }
}

/// Prints the given message in the VGA Buffer
pub fn print_vga(msg: &[u8]) {
    let vga_pointer = VGA_ADDR as *mut u8;

    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            *vga_pointer.offset(i as isize * 2) = byte;
            *vga_pointer.offset(i as isize * 2 + 1) = DEFAULT_COLOR;
        }
    }
}

/// The VGA screen matrix
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A printed char in the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    /// Create a new [`ScreenChar`]
    fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        Self {
            ascii_character,
            color_code,
        }
    }
}

/// A full color code to print
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Create a new [`ColorCode`] from its components
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}
