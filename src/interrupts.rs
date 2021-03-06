use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use crate::gdt;
use lazy_static::lazy_static;

// IDT has to have a static lifetime and we wrap it in lazy_static in case we want
// to read runtime values and to abstract the unsafetyness of a mutable static
lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe {
      idt.double_fault.set_handler_fn(double_fault_handler)
          .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
  };
}

pub fn init_idt() {
  IDT.load(); // Load IDT into CPU using lidt instruction
}

extern "x86-interrupt" fn breakpoint_handler(
  stack_frame: &mut InterruptStackFrame)
{
  println!("EXCEPTION BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
  stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
  panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame)
}


#[cfg(test)]
use crate::{serial_print, serial_println};

#[test_case]
fn test_breakpoint_exception() {
  serial_print!("test_breakpoint_exception...");
  x86_64::instructions::interrupts::int3();
  serial_println!("[ok]");
}
