use sloggrs::{can_log, LogLevels};

/// Test if can_log works as expected
#[test]
fn test_can_log() {
    sloggrs::init!(DEBUG);
    assert_eq!(can_log(LogLevels::DEBUG), true);

    sloggrs::init!(FATAL);
    assert_eq!(can_log(LogLevels::FATAL), true);
    assert_eq!(can_log(LogLevels::ERROR), false);
}
