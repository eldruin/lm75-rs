extern crate embedded_hal_mock as hal;
use lm75::{Error, FaultQueue, Lm75, OsMode, OsPolarity, SlaveAddr};

const DEVICE_BASE_ADDRESS: u8 = 0b100_1000;

struct Register;

impl Register {
    const TEMPERATURE: u8 = 0x00;
    const CONFIGURATION: u8 = 0x01;
    const T_HYST: u8 = 0x02;
    const T_OS: u8 = 0x03;
}

fn setup<'a>(data: &'a [u8]) -> Lm75<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Lm75::new(dev, SlaveAddr::default())
}

fn check_sent_data(sensor: Lm75<hal::I2cMock>, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_BASE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
}

fn assert_invalid_input_data_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidInputData) => (),
        _ => panic!("Did not return Error::InvalidInputData."),
    }
}

#[test]
fn can_create() {
    setup(&[0]);
}

#[test]
fn can_enable() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    check_sent_data(dev, &[Register::CONFIGURATION, 0]);
}

#[test]
fn can_disable() {
    let mut dev = setup(&[0]);
    dev.disable().unwrap();
    check_sent_data(dev, &[Register::CONFIGURATION, 1]);
}

#[test]
fn can_read_temperature() {
    let mut dev = setup(&[0b1110_0111, 0b1010_0101]);
    let temp = dev.read_temperature().unwrap();
    assert_eq!(-24.5, temp);
    check_sent_data(dev, &[Register::TEMPERATURE]);
}

macro_rules! set_config_test {
    ( $test_name:ident, $method:ident, $value:expr, $expected:expr ) => {
        #[test]
        fn $test_name() {
            let mut dev = setup(&[0]);
            dev.$method($value).unwrap();
            check_sent_data(dev, &[Register::CONFIGURATION, $expected]);
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
            let mut dev = setup(&[0]);
            dev.$method($value).unwrap();
            check_sent_data(dev, &[$register, $expected_msb, $expected_lsb]);
        }
    };
}

set_temp_test!(
    can_set_os_temp_0_5,
    set_os_temperature,
    0.5,
    Register::T_OS,
    0b0000_0000,
    1
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
            let mut dev = setup(&[0]);
            assert_invalid_input_data_error(dev.$method($value));
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
    1
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
