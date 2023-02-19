#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bgos::{qemu, serial_print, serial_println};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  bgos::test::panic_handler::panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  serial_print!("stack_overflow::stack_overflow...\t");

  bgos::global_descriptor_table::init();
  init_test_idt();

  stack_overflow();

  panic!("Execution continued after stack overflow");
}

lazy_static! {
  static ref TEST_IDT: InterruptDescriptorTable = {
    let mut table = InterruptDescriptorTable::new();
    unsafe {
      table
        .double_fault
        .set_handler_fn(test_double_fault_handler)
        .set_stack_index(bgos::global_descriptor_table::DOUBLE_FAULT_STACK_INDEX);
    }

    table
  };
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
  serial_println!("[OK]");

  qemu::exit_qemu(qemu::QemuExitCode::Success);

  loop {}
}

fn init_test_idt() {
  TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow();
  volatile::Volatile::new(0).read(); // Prevent tail call elimination (if any)
}
