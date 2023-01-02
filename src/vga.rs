use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

type CodePage437 = u8;
type VgaCharacter = CodePage437;

const VGA_MEMORY_MAP_ADDRESS: usize = 0xb8000;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const LAST_ROW: usize = BUFFER_HEIGHT - 1;

const ASCII_PRINTABLE_START: VgaCharacter = 0x20;
const ASCII_PRINTABLE_END: VgaCharacter = 0x7E;
const ASCII_NEWLINE: VgaCharacter = b'\n';
const ASCII_SPACE: VgaCharacter = b' ';

const CHAR_UNPRINTABLE: VgaCharacter = 0xFE;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Color {
  Black = 0x0,
  Blue = 0x1,
  Green = 0x2,
  Cyan = 0x3,
  Red = 0x4,
  Magenta = 0x5,
  Brown = 0x6,
  LightGray = 0x7,
  DarkGray = 0x8,
  LightBlue = 0x9,
  LightGreen = 0xA,
  LightCyan = 0xB,
  LightRed = 0xC,
  Pink = 0xD,
  Yellow = 0xE,
  White = 0xF,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
struct ScreenChar {
  character_code: VgaCharacter,
  color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

struct Writer {
  column_position: usize,
  color_code: ColorCode,
  buffer: &'static mut Buffer,
}

impl Writer {
  fn write_string(&mut self, string: &str) {
    for character_code in string.bytes() {
      match character_code {
        ASCII_PRINTABLE_START..=ASCII_PRINTABLE_END | ASCII_NEWLINE => self.write_character(character_code),
        _ => self.write_character(CHAR_UNPRINTABLE),
      }
    }
  }

  fn write_character(&mut self, character_code: VgaCharacter) {
    match character_code {
      ASCII_NEWLINE => self.new_line(),
      character_code => {
        if self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = LAST_ROW;
        let col = self.column_position;

        let color_code = self.color_code;

        self.buffer.chars[row][col].write(ScreenChar {
          character_code,
          color_code,
        });
        self.column_position += 1;
      }
    };
  }

  fn new_line(&mut self) {
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read());
      }
    }

    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[LAST_ROW][col].write(ScreenChar {
        character_code: ASCII_SPACE,
        color_code: self.color_code,
      });
    }

    self.column_position = 0;
  }
}

impl fmt::Write for Writer {
  fn write_str(&mut self, string: &str) -> fmt::Result {
    self.write_string(string);
    Ok(())
  }
}

lazy_static! {
  static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(VGA_MEMORY_MAP_ADDRESS as *mut Buffer) },
  });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use core::fmt::Write;
  WRITER.lock().write_fmt(args).unwrap();
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
