#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failure = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
  use x86_64::instructions::port::Port;
  const QEMU_EXIT_CODE_PORT: u16 = 0xF4;
  type QemuExitCodeInt = u32;

  let mut port = Port::new(QEMU_EXIT_CODE_PORT);
  unsafe {
    port.write(exit_code as QemuExitCodeInt);
  }
}
