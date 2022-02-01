//! Logging macros

#[macro_export]
/// Write a debug message
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!(
            DEBUG,
            "\x1b[95m[DEBUG] [{}:{}:{}] {}\x1b[m",
            file!(),
            line!(),
            column!(),
            $($arg)*
        )
    };
}

#[macro_export]
/// Write a warning
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!(
            WARN,
            "\x1b[93m[{}:{}:{}] {}\x1b[m",
            file!(),
            line!(),
            column!(),
            $($arg)*
        )
    };
}

#[macro_export]
/// Write an information
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!(
            INFO,
            "\x1b[96m{}\x1b[m",
            $($arg)*
        )
    };
}

#[macro_export]
/// Write an error
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!(
            ERROR,
            "\x1b[91m{}\x1b[m",
            $($arg)*
        )
    };
}

#[macro_export]
/// Write a fatal message
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log!(
            FATAL,
            "\x1b[1;21;91m[{}] [{}:{}:{}] [FATAL] {}\x1b[0;0;m",
            module_path!(),
            file!(),
            line!(),
            column!(),
            $($arg)*
        )
    };
}
