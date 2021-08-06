use embedded_hal_mock::i2c::Transaction as I2cTrans;
use lm75::{FaultQueue, OsMode, OsPolarity,ic};

mod common;

use crate::common::{assert_invalid_input_data_error, assert_invalid_register_error, destroy, destroy_pct2075,new, new_pct2075, Register, ADDR};

#[test]
fn can_create_and_destroy_new() {
    let sensor = new(&[]);
    destroy(sensor);
}

#[test]
fn can_create_and_destroy_new_pct2075() {
    let sensor = new_pct2075(&[]);
    destroy_pct2075(sensor);
}

#[test]
fn can_enable() {
    let mut sensor = new(&[I2cTrans::write(ADDR, vec![Register::CONFIGURATION, 0])]);
    sensor.enable().unwrap();
    destroy(sensor);
}

#[test]
fn can_disable() {
    let mut sensor = new(&[I2cTrans::write(ADDR, vec![Register::CONFIGURATION, 1])]);
    sensor.disable().unwrap();
    destroy(sensor);
}

#[test]
fn can_read_temperature() {
    let mut sensor = new(&[I2cTrans::write_read(
        ADDR,
        vec![Register::TEMPERATURE],
        vec![0b1110_0111, 0b1010_0101], // -24.5
    )]);
    let temp = sensor.read_temperature().unwrap();
    assert!(-24.4 > temp);
    assert!(-24.6 < temp);
    destroy(sensor);
}

#[test]
fn can_read_sample_rate() {
    let mut sensor = new_pct2075(&[I2cTrans::write_read(
        ADDR,
        vec![Register::T_IDLE],
        vec![0b0000_0001], // 100ms
    )]);
    let period = sensor.read_sample_rate().unwrap();
    assert_eq!(100, period);
    destroy(sensor);
}

macro_rules! set_config_test {
    ( $test_name:ident, $method:ident, $value:expr, $expected:expr ) => {
        #[test]
        fn $test_name() {
            let mut sensor = new(&[I2cTrans::write(
                ADDR,
                vec![Register::CONFIGURATION, $expected],
            )]);
            sensor.$method($value).unwrap();
            destroy(sensor);
        }
    };
}

set_config_test!(
    can_set_fault_queue_1,
    set_fault_queue,
    FaultQueue::_1,
    0b0000_0000
);
set_config_test!(
    can_set_fault_queue_2,
    set_fault_queue,
    FaultQueue::_2,
    0b0000_1000
);
set_config_test!(
    can_set_fault_queue_4,
    set_fault_queue,
    FaultQueue::_4,
    0b0001_0000
);
set_config_test!(
    can_set_fault_queue_6,
    set_fault_queue,
    FaultQueue::_6,
    0b0001_1000
);

set_config_test!(
    can_set_os_polarity_low,
    set_os_polarity,
    OsPolarity::ActiveLow,
    0b0000_0000
);
set_config_test!(
    can_set_os_polarity_high,
    set_os_polarity,
    OsPolarity::ActiveHigh,
    0b0000_0100
);

set_config_test!(
    can_set_os_mode_low,
    set_os_mode,
    OsMode::Comparator,
    0b0000_0000
);
set_config_test!(
    can_set_os_mode_high,
    set_os_mode,
    OsMode::Interrupt,
    0b0000_0010
);

macro_rules! set_temp_test {
    ( $test_name:ident, $method:ident, $value:expr, $register:expr,
      $expected_msb:expr, $expected_lsb:expr ) => {
        #[test]
        fn $test_name() {
            let mut sensor = new(&[I2cTrans::write(
                ADDR,
                vec![$register, $expected_msb, $expected_lsb],
            )]);
            sensor.$method($value).unwrap();
            destroy(sensor);
        }
    };
}

set_temp_test!(
    can_set_os_temp_0_5,
    set_os_temperature,
    0.5,
    Register::T_OS,
    0b0000_0000,
    0b1000_0000
);
set_temp_test!(
    can_set_os_temp_min,
    set_os_temperature,
    -55.0,
    Register::T_OS,
    0b1100_1001,
    0
);
set_temp_test!(
    can_set_os_temp_max,
    set_os_temperature,
    125.0,
    Register::T_OS,
    0b0111_1101,
    0
);

macro_rules! invalid_temp_test {
    ($test_name:ident, $method:ident, $value:expr) => {
        #[test]
        fn $test_name() {
            let mut sensor = new(&[]);
            assert_invalid_input_data_error(sensor.$method($value));
        }
    };
}

invalid_temp_test!(set_os_temperature_too_low, set_os_temperature, -55.5);
invalid_temp_test!(set_os_temperature_too_high, set_os_temperature, 125.5);

set_temp_test!(
    can_set_hyst_temp_0_5,
    set_hysteresis_temperature,
    0.5,
    Register::T_HYST,
    0b0000_0000,
    0b1000_0000
);
set_temp_test!(
    can_set_hyst_temp_min,
    set_hysteresis_temperature,
    -55.0,
    Register::T_HYST,
    0b1100_1001,
    0
);
set_temp_test!(
    can_set_hyst_temp_max,
    set_hysteresis_temperature,
    125.0,
    Register::T_HYST,
    0b0111_1101,
    0
);

invalid_temp_test!(
    set_hyst_temperature_too_low,
    set_hysteresis_temperature,
    -55.5
);
invalid_temp_test!(
    set_hyst_temperature_too_high,
    set_hysteresis_temperature,
    125.5
);

macro_rules! set_sample_rate_test {
    ( $test_name:ident, $method:ident, $value:expr, $register:expr,
      $period:expr) => {
        #[test]
        fn $test_name() {
            let mut sensor = new_pct2075(&[I2cTrans::write(
                ADDR,
                vec![$register, $period],
            )]);
            sensor.$method($value).unwrap();
            destroy_pct2075(sensor);
        }
    };
}

set_sample_rate_test!(
    can_set_max_sample_rate,
    set_sample_rate,
    3100,
    Register::T_IDLE,
    0b0001_1111
);
set_sample_rate_test!(
    can_set_custom_sample_rate,
    set_sample_rate,
    1500,
    Register::T_IDLE,
    0b0000_1111
);
set_sample_rate_test!(
    can_set_default_sample_rate,
    set_sample_rate,
    100,
    Register::T_IDLE,
    0b0000_0001
);

macro_rules! invalid_sample_rate_test {
    ($test_name:ident, $method:ident, $value:expr) => {
        #[test]
        fn $test_name() {
            let mut sensor = new_pct2075<IC>(&[]);
            assert_invalid_input_data_error(sensor.$method($value));
        }
    };
}

invalid_sample_rate_test!(set_sample_rate_too_high, set_sample_rate, 4000);
invalid_sample_rate_test!(set_non_multiple_sample_rate, set_sample_rate, 1234);
