//! Sloggrs - Simple logger

pub mod level;
pub mod macros;
pub use level::LogLevels;
use std::sync::atomic::{AtomicU8, Ordering};

pub static LOG_LEVEL: AtomicU8 = AtomicU8::new(0);

/**
 * Init logger with level (default level: INFO)
*/
#[macro_export]
macro_rules! init {
    () => {
        $crate::init_with_level(None)
    };
    ($arg:tt) => {
        $crate::init_with_level(Some($crate::LogLevels::$arg))
    };
}

/**
 * Initialise logger with level
 */
#[doc(hidden)]
pub fn init_with_level(loglevel: Option<LogLevels>) {
    if let Some(level) = loglevel {
        LOG_LEVEL.store(level as u8, Ordering::Relaxed);
    } else {
        LOG_LEVEL.store(LogLevels::INFO as u8, Ordering::Relaxed);
    }
}

/**
 * Base logging macro
 */
#[macro_export]
macro_rules! log {
    ($level:tt, $($arg:tt)*) => {
        if $crate::can_log($crate::LogLevels::$level) {
            println!("{}", format!($($arg)*))
        }
    };
}

/**
 * Get log level
*/
pub fn get_log_level() -> LogLevels {
    LogLevels::from(LOG_LEVEL.load(Ordering::Relaxed))
}

/**
 * Check if we can log `level`
*/
pub fn can_log(level: LogLevels) -> bool {
    level as u8 >= get_log_level() as u8
}
