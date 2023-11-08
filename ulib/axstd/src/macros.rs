//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}

#[macro_export]
macro_rules! pinfo {
    () => { $crate::__pinfo_impl(format_args!($($arg)*)) };
    ($($arg:tt)*) => {
        if 2 >= $crate::get_log_level() {
            $crate::io::__pinfo_impl(format_args!("{}\n", format_args!($($arg)*)));
        }
    }
}

#[macro_export]
macro_rules! pdebug {
    () => { $crate::__pdebug_impl(format_args!($($arg)*)) };
    ($($arg:tt)*) => {
        if 1 >= $crate::get_log_level() {
            $crate::io::__pdebug_impl(format_args!("{}\n", format_args!($($arg)*)));
        }
        // $crate::io::__pdebug_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}
