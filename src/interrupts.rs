use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
  static ref INTERRUPT_TABLE: InterruptDescriptorTable = {
    let mut table = InterruptDescriptorTable::new();
    table.breakpoint.set_handler_fn(breakpoint_handler);

    table
  };
}

pub fn init_interrupt_table() {
  INTERRUPT_TABLE.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
  println!("Exception: breakpoint\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
  x86_64::instructions::interrupts::int3();
}
