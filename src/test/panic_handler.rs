use crate::qemu::{exit_qemu, QemuExitCode};
use crate::serial_println;
use core::panic::PanicInfo;

pub fn panic_handler(info: &PanicInfo) -> ! {
  serial_println!("[FAIL]");
  serial_println!();
  serial_println!("Error: {}", info);
  serial_println!();

  exit_qemu(QemuExitCode::Failure);

  loop {}
}
