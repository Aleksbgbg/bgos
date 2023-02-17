#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bgos::test::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bgos::println;

#[no_mangle]
extern "C" fn _start() -> ! {
  test_main();

  loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  bgos::test::panic_handler::panic_handler(info)
}

#[test_case]
fn test_println() {
  println!("test output");
}
