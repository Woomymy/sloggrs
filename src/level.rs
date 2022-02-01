#[derive(Debug)]
pub enum LogLevels {
    DEBUG = 0,
    WARN = 1,
    INFO = 2,
    ERROR = 3,
    FATAL = 4,
}

impl From<u8> for LogLevels {
    fn from(i: u8) -> Self {
        use LogLevels::*;
        match i {
            0 => DEBUG,
            1 => WARN,
            2 => INFO,
            3 => ERROR,
            4 => FATAL,
            _ => DEBUG,
        }
    }
}
