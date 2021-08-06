use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use lm75::{Error, Lm75, Address, ic};

pub const ADDR: u8 = 0b100_1000;

pub struct Register;

impl Register {
    pub const TEMPERATURE: u8 = 0x00;
    pub const CONFIGURATION: u8 = 0x01;
    pub const T_HYST: u8 = 0x02;
    pub const T_OS: u8 = 0x03;
    pub const T_IDLE: u8 = 0x04;
}

pub fn new<IC>(transactions: &[I2cTrans]) -> Lm75<I2cMock, ic::Lm75> {
    Lm75::new(I2cMock::new(transactions), Address::default())
}

pub fn new_pct2075<IC>(transactions: &[I2cTrans]) -> Lm75<I2cMock, ic::Pct2075> {
    Lm75::new_pct2075(I2cMock::new(transactions), Address::default())
}

pub fn destroy<IC>(sensor: Lm75<I2cMock, ic::Lm75>) {
    sensor.destroy().done();
}

pub fn destroy_pct2075<IC>(sensor: Lm75<I2cMock, ic::Pct2075>) {
    sensor.destroy().done();
}

pub fn assert_invalid_input_data_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidInputData) => (),
        _ => panic!("Did not return Error::InvalidInputData."),
    }
}
