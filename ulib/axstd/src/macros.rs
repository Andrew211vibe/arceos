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
    ($($arg:tt)*) => {
        let level = option_env!("debug").unwrap_or("0").parse::<u8>().unwrap();
        if level < 1 {
            $crate::io::__print_impl(format_args!("\u{1B}[97m[INFO] [user] {}\u{1B}[0m\n", format_args!($($arg)*)));
        }
    }
}

#[macro_export]
macro_rules! pdev {
    ($($arg:tt)*) => {
        let level = option_env!("debug").unwrap_or("0").parse::<u8>().unwrap();
        if level < 2 {
            $crate::io::__print_impl(format_args!("\u{1B}[96m[DEV] [user] {}\u{1B}[0m\n", format_args!($($arg)*)));
        }
    }
}

#[macro_export]
macro_rules! pdebug {
    ($($arg:tt)*) => {
        let level = option_env!("debug").unwrap_or("0").parse::<u8>().unwrap();
        if level < 3 {
            $crate::io::__print_impl(format_args!("\u{1B}[95m[DEBUG] [user] {}\u{1B}[0m\n", format_args!($($arg)*)));
        }
    }
}