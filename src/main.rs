#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[allow(dead_code)]
mod output;
#[allow(dead_code)]
mod qemu;

#[cfg(test)]
mod test;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  println!("{}", info);

  loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  #[cfg(not(test))]
  main();

  #[cfg(test)]
  test_main();

  loop {}
}

fn main() {
  println!("Hello, world!");
}
