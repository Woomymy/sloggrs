//! Sloggrs - Simple logger

pub mod level;

use std::sync::atomic::{AtomicU8, Ordering};

pub use level::LogLevels;

pub static LOG_LEVEL: AtomicU8 = AtomicU8::new(0);

/**
 * Init logger with level (default level: INFO)
*/
#[macro_export]
macro_rules! init {
    () => {
        $crate::init(None)
    };
    ($arg:tt) => {
        $crate::init(Some($crate::LogLevels::$arg))
    };
}
/**
 * Initialise logger
 */
pub fn init(loglevel: Option<LogLevels>) {
    if let Some(level) = loglevel {
        LOG_LEVEL.store(level as u8, Ordering::Relaxed);
    } else {
        LOG_LEVEL.store(LogLevels::INFO as u8, Ordering::Relaxed);
    }
}

/**
 * Get log level
*/
pub fn get_log_level() -> LogLevels {
    LogLevels::from(LOG_LEVEL.load(Ordering::Relaxed))
}
