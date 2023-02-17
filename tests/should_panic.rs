#![no_std]
#![no_main]

use bgos::qemu::{self, QemuExitCode};
use bgos::{serial_print, serial_println};
use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
  should_fail();
  serial_print!("[FAIL: NO PANIC]");
  qemu::exit_qemu(QemuExitCode::Failure);

  loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[OK]");
  qemu::exit_qemu(QemuExitCode::Success);

  loop {}
}

fn should_fail() {
  serial_print!("should_panic::should_fail...\t");
  assert!(false);
}
