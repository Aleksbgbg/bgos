use crate::qemu::{exit_qemu, QemuExitCode};
use crate::serial_println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  serial_println!("[FAIL]");
  serial_println!();
  serial_println!("Error: {}", info);
  serial_println!();

  exit_qemu(QemuExitCode::Failure);

  loop {}
}
