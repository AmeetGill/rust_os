use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::LazyStatic;

lazy_static{
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0xf4) };
        serial_port.init();
        Mutex::new(serial_port)
    }
};

#[doc(hidder)]
pub fn _print(arg: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(arg).unwrap();
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_exports]
macro_rules!  {
    () => $crate::serial_print!("\n");
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt,"\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));   
}