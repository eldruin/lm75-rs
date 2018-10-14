extern crate lm75;
extern crate embedded_hal_mock as hal;
use lm75::{ LM75, SlaveAddr };

const DEVICE_BASE_ADDRESS: u8 = 0b100_1000;

fn setup<'a>(data: &'a[u8]) -> LM75<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    LM75::new(dev, SlaveAddr::default())
}

fn check_sent_data(sensor: LM75<hal::I2cMock>, data: &[u8]) {
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
    check_sent_data(dev, &[0]);
}

#[test]
fn can_disable() {
    let mut dev = setup(&[0]);
    dev.disable().unwrap();
    check_sent_data(dev, &[1]);
}