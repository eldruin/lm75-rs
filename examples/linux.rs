use linux_embedded_hal::I2cdev;
use lm75::{Address, Lm75};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = Address::default();
    let mut sensor = Lm75::new(dev, address);
    let temperature = sensor.read_temperature().unwrap();
    println!("Temperature: {}", temperature);
}
