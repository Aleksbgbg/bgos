use crate::output::vga;
use crate::{assert_starts_with, print, println};

#[test_case]
fn test_simple() {
  println!("test_println_simple output");
}

#[test_case]
fn test_many() {
  const TEST_LINES: usize = 200;

  for _ in 0..TEST_LINES {
    println!("test_println_many output");
  }
}

#[test_case]
fn test_correctly_written() {
  const STRING: &str = "long test string fitting on a single line";
  const LAST_LINE: isize = -1;

  print!("{}", STRING);

  assert_starts_with!(STRING, vga::vga_read_line(LAST_LINE, &mut vga::vga_line_buffer()));
}
