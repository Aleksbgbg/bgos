use crate::qemu::{exit_qemu, QemuExitCode};
use crate::{serial_print, serial_println};

pub trait Testable {
  fn run(&self) -> ();
}

fn crop_letters(string: &str, up_to: usize) -> &str {
  match string.char_indices().skip(up_to).next() {
    Some((position, _)) => &string[position..],
    None => "",
  }
}

impl<T> Testable for T
where
  T: Fn(),
{
  fn run(&self) {
    const UNIT_TEST_PREFIX_LEN: usize = "bgos::test::units::".len();
    let type_name = crop_letters(core::any::type_name::<T>(), UNIT_TEST_PREFIX_LEN);

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

  exit_qemu(QemuExitCode::Success);
}
