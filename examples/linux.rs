use linux_embedded_hal::I2cdev;
use lm75::{Lm75, SlaveAddr};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Lm75::new(dev, SlaveAddr::default());
    let temperature = sensor.read_temperature().unwrap();
    println!("Temperature: {}", temperature);
}
