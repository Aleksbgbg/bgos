use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
  static ref SERIAL_0: Mutex<SerialPort> = {
    const SERIAL_INTERFACE_0: u16 = 0x3F8;

    let mut serial_port = unsafe { SerialPort::new(SERIAL_INTERFACE_0) };
    serial_port.init();

    Mutex::new(serial_port)
  };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use fmt::Write;
  SERIAL_0.lock().write_fmt(args).expect("Unable to print to SERIAL_0.");
}

#[macro_export]
macro_rules! serial_print {
  ($($arg:tt)*) => ($crate::output::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
  () => ($crate::serial_print!("\n"));
  ($format:expr) => ($crate::serial_print!(concat!($format, "\n")));
  ($format:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($format, "\n"), $($arg)*));
}
