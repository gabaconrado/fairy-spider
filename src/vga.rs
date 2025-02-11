use spin::Mutex;
use volatile::Volatile;

/// A default [`ColorCode`] to be used by the system
pub const DEFAULT_COLOR_CODE: ColorCode = ColorCode::new(Color::Yellow, Color::Black);
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

/// Prints text to the screen using the static [`Writer`]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

/// Prints text to the screen with a new line using the static [`Writer`]
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    // This is fine because our write method never errors
    let _ = WRITER.lock().write_fmt(args);
}

lazy_static::lazy_static! {
    /// A static Writer that can be used without requiring it to be always created
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::initialize(DEFAULT_COLOR_CODE));
}

// TODO: Remove this lint
#[allow(dead_code)]
/// All available colors to print
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// Code for the color Black
    Black = 0,
    /// Code for the color Blue
    Blue = 1,
    /// Code for the color Green
    Green = 2,
    /// Code for the color Cyan
    Cyan = 3,
    /// Code for the color Red
    Red = 4,
    /// Code for the color Magenta
    Magenta = 5,
    /// Code for the color Brown
    Brown = 6,
    /// Code for the color Light Gray
    LightGray = 7,
    /// Code for the color Dark Gray
    DarkGray = 8,
    /// Code for the color Light Blue
    LightBlue = 9,
    /// Code for the color Light Green
    LightGreen = 10,
    /// Code for the color Light Cyan
    LightCyan = 11,
    /// Code for the color Light Red
    LightRed = 12,
    /// Code for the color Pink
    Pink = 13,
    /// Code for the color Yellow
    Yellow = 14,
    /// Code for the color White
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
    /// Initialize a new [`Writer`]
    pub fn initialize(color_code: ColorCode) -> Self {
        let column_position = 0;
        let buffer = unsafe { &mut *(VGA_ADDR as *mut Buffer) };
        Self {
            column_position,
            color_code,
            buffer,
        }
    }

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

                self.buffer.chars[row][col].write(ScreenChar::new(byte, color_code));

                self.column_position += 1;
            }
        }
    }

    /// Adds a new line to the current position
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

    /// Clears the given row from the display
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// The VGA screen matrix
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
pub struct ColorCode(u8);

impl ColorCode {
    /// Create a new [`ColorCode`] from its components
    const fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}
