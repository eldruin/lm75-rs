use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use lm75::{Error, Lm75, SlaveAddr};

pub const ADDR: u8 = 0b100_1000;

pub struct Register;

impl Register {
    pub const TEMPERATURE: u8 = 0x00;
    pub const CONFIGURATION: u8 = 0x01;
    pub const T_HYST: u8 = 0x02;
    pub const T_OS: u8 = 0x03;
}

pub fn new(transactions: &[I2cTrans]) -> Lm75<I2cMock> {
    Lm75::new(I2cMock::new(transactions), SlaveAddr::default())
}

pub fn destroy(sensor: Lm75<I2cMock>) {
    sensor.destroy().done();
}

pub fn assert_invalid_input_data_error<T, E>(result: Result<T, Error<E>>) {
    match result {
        Err(Error::InvalidInputData) => (),
        _ => panic!("Did not return Error::InvalidInputData."),
    }
}
