extern crate lm75;
extern crate embedded_hal_mock as hal;
use lm75::{ Lm75, SlaveAddr };

const DEVICE_BASE_ADDRESS: u8 = 0b100_1000;

struct Register;

impl Register {
    const TEMPERATURE   : u8 = 0x00;
    const CONFIGURATION : u8 = 0x01;
}

fn setup<'a>(data: &'a[u8]) -> Lm75<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    Lm75::new(dev, SlaveAddr::default())
}

fn check_sent_data(sensor: Lm75<hal::I2cMock>, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(DEVICE_BASE_ADDRESS));
    assert_eq!(dev.get_write_data(), &data[..]);
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
