// Module to pass messages from our OS to terminal using a serial port.
// We send messages to the serial port, and setting a QEMU flag prints messages
// we sent to the port to stdio

use spin::Mutex;
use uart_16550::SerialPort;
use lazy_static::lazy_static;


lazy_static! {
  pub static ref SERIAL1: Mutex<SerialPort> = {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
  };
}


#[macro_export]
macro_rules! serial_print {
  ($($arg:tt)*) =>  {$crate::serial::_print(format_args!($($arg)*))};
}

#[macro_export]
macro_rules! serial_println {
  ()                       => {$crate::serial_print!("\n")};
  ($fmt:expr)              => {$crate::serial_print!(concat!($fmt, "\n"))};
  ($fmt:expr, $($arg:tt)*) => {$crate::serial_print!(concat!($fmt, "\n"), $($arg)*)};
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
  use core::fmt::Write;
  SERIAL1.lock().write_fmt(args).expect("Unable to print to serial.");
}
