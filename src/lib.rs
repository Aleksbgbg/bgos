#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[allow(dead_code)]
pub mod output;
#[allow(dead_code)]
pub mod qemu;
#[allow(unused_imports)]
pub mod test;

#[cfg(test)]
#[no_mangle]
extern "C" fn _start() -> ! {
  test_main();

  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  test::panic_handler::panic_handler(info);
}
