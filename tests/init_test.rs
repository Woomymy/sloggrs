use sloggrs::*;
/// Tests if init!() works as expected
#[test]
fn test_init_macro_default() {
    // Check if default log level is info
    init!();
    assert_eq!(get_log_level(), LogLevels::INFO);
}

/// Tests if init!(level) works as expected
#[test]
fn test_init_macro_param() {
    init!(FATAL);
    assert_eq!(get_log_level(), LogLevels::FATAL);
}
