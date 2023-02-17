#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bgos::test::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bgos::println;
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
  #[cfg(not(test))]
  main();

  #[cfg(test)]
  test_main();

  loop {}
}

fn main() {
  println!("Hello, world!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  bgos::test::panic_handler::panic_handler(info)
}
