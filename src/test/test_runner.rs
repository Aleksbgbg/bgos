use crate::qemu::{exit_qemu, QemuExitCode};
use crate::{serial_print, serial_println};

pub trait Testable {
  fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
  fn run(&self) {
    let type_name = core::any::type_name::<T>();

    serial_print!("  {:<60}", type_name);
    self();
    serial_println!("[OK]");
  }
}

pub fn test_runner(tests: &[&dyn Testable]) {
  serial_println!();
  match tests.len() {
    0 => serial_println!("No tests to run (0)."),
    1 => serial_println!("Running 1 test:"),
    len => serial_println!("Running {} tests:", len),
  }

  for test in tests {
    test.run();
  }

  serial_println!();

  exit_qemu(QemuExitCode::Success);
}
