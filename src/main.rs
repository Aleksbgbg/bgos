#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  print_vga();

  loop {}
}

fn print_vga() {
  const BACKGROUND_CYAN: u8 = 0xb;
  const HELLO_WORLD_STRING: &[u8] = b"Hello, world!";

  let vga_buffer_pointer = 0xb8000 as *mut u8;

  for (index, &character) in HELLO_WORLD_STRING.iter().enumerate() {
    unsafe {
      *vga_buffer_pointer.offset((index * 2) as isize) = character;
      *vga_buffer_pointer.offset(((index * 2) + 1) as isize) = BACKGROUND_CYAN;
    }
  }
}
