use core::fmt::{self, Write};

use crate::{sync::SpinLock, uart};

struct Console;

impl Write for Console {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            uart::putchar(byte);
        }
        Ok(())
    }
}

static PRINT_LOCK: SpinLock = SpinLock::new();

pub fn print(arguments: fmt::Arguments<'_>) {
    let _guard = PRINT_LOCK.lock();
    Console.write_fmt(arguments).ok();
}

macro_rules! println {
    () => {
        $crate::console::print(format_args!("\n"))
    };
    ($format:expr $(, $argument:expr)* $(,)?) => {
        $crate::console::print(format_args!(concat!($format, "\n") $(, $argument)*))
    };
}
