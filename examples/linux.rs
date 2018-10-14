extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate lm75;

use linux_embedded_hal::I2cdev;
use lm75::{ LM75, SlaveAddr };

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let _sensor = LM75::new(dev, SlaveAddr::default());
    //println!("Temperature: {}", reading);
}
