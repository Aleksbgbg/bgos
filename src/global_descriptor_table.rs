use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

pub const DOUBLE_FAULT_STACK_INDEX: u16 = 0;

lazy_static! {
  static ref TASK_STATE_SEGMENT: TaskStateSegment = {
    let mut task_state_segment = TaskStateSegment::new();
    task_state_segment.interrupt_stack_table[DOUBLE_FAULT_STACK_INDEX as usize] = {
      const PAGE_SIZE: usize = 4096;
      const STACK_SIZE: usize = PAGE_SIZE * 5;

      static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

      let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
      let stack_end = stack_start + STACK_SIZE;

      stack_end
    };

    task_state_segment
  };
}

struct Selectors {
  code_selector: SegmentSelector,
  tss_selector: SegmentSelector,
}

lazy_static! {
  static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, Selectors) = {
    let mut table = GlobalDescriptorTable::new();

    let code_selector = table.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = table.add_entry(Descriptor::tss_segment(&TASK_STATE_SEGMENT));

    (
      table,
      Selectors {
        code_selector,
        tss_selector,
      },
    )
  };
}

pub fn init() {
  use x86_64::instructions::tables::load_tss;
  use x86_64::registers::segmentation::{Segment, CS};

  GLOBAL_DESCRIPTOR_TABLE.0.load();
  unsafe {
    CS::set_reg(GLOBAL_DESCRIPTOR_TABLE.1.code_selector);
    load_tss(GLOBAL_DESCRIPTOR_TABLE.1.tss_selector);
  }
}
