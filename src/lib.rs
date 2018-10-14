//! This is a platform agnostic Rust driver for the LM75 temperature
//! sensor and thermal watchdog, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - TODO
//!
//! ## The device
//! The LM75 temperature sensor includes a delta-sigma analog-to-digital
//! converter, and a digital overtemperature detector. The host can
//! query the LM75 through its I2C interface to read temperature at any
//! time. The open-drain overtemperature output (OS) sinks current when
//! the programmable temperature limit is exceeded.
//! The OS output operates in either of two modes, comparator or
//! interrupt. The host controls the temperature at which the alarm is
//! asserted (TOS) and the hysteresis temperature below which the alarm
//! condition is not valid (THYST). Also, the LM75's TOS and THYST
//! registers can be read by the host. The address of the LM75 is set
//! with three pins to allow multiple devices to work on the same bus.
//! Power-up is in comparator mode, with defaults of TOS= +80ºC and
//! THYST= +75ºC. The 3.0V to 5.5V supply voltage range, low supply
//! current, and I2C interface make the LM75 ideal for many applications
//! in thermal management and protection.
//!
//! Datasheet:
//! - [LM75](https://datasheets.maximintegrated.com/en/ds/LM75.pdf)
//!
//! This driver is also compatible with LM75B and LM75C: [LM75B/C Datasheet]
//!
//! [LM75B/C Datasheet]: http://www.ti.com/lit/ds/symlink/lm75b.pdf
//!

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c::Write;
extern crate bit_field;
use bit_field::BitField;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Possible slave addresses
#[derive(Debug, Clone)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit values for A2, A1 and A0
    Alternative(bool, bool, bool)
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a2, a1, a0) => default           |
                                                  ((a2 as u8) << 2) |
                                                  ((a1 as u8) << 1) |
                                                    a0 as u8
        }
    }
}
const DEVICE_BASE_ADDRESS: u8 = 0b100_1000;

struct Register;

impl Register {
    const TEMPERATURE   : u8 = 0x00;
    const CONFIGURATION : u8 = 0x01;
    const T_HYST        : u8 = 0x02;
    const T_OS          : u8 = 0x03;
}


struct BitFlags;

impl BitFlags {
    const SHUTDOWN     : usize = 0;
    const COMP_INT     : usize = 1;
    const OS_POLARITY  : usize = 2;
    const FAULT_QUEUE0 : usize = 3;
    const FAULT_QUEUE1 : usize = 4;
}

/// LM75 device driver.
#[derive(Debug, Default)]
pub struct LM75<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: u8,
}

impl<I2C, E> LM75<I2C>
where
    I2C: Write<Error = E>
{
    /// Create new instance of the LM75 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        LM75 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: 0
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the sensor.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let mut config = self.config;
        config.set_bit(BitFlags::SHUTDOWN, false);
        self.write_config(config)
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let mut config = self.config;
        config.set_bit(BitFlags::SHUTDOWN, true);
        self.write_config(config)
    }

    fn write_config(&mut self, config: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[config])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}

fn convert_temp(msb: u8, lsb: u8) -> f32 {
    let value = ( msb.get_bit(0) as u16
                + msb.get_bit(1) as u16 * 2
                + msb.get_bit(2) as u16 * 4
                + msb.get_bit(3) as u16 * 8
                + msb.get_bit(4) as u16 * 16
                + msb.get_bit(5) as u16 * 32
                + msb.get_bit(6) as u16 * 64) as f32
                + (lsb.get_bit(7) as u8 as f32 * 0.5);
    
    if msb.get_bit(7) {
        -value
    }
    else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0b100_1000, SlaveAddr::Alternative(false, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1001, SlaveAddr::Alternative(false, false,  true).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1010, SlaveAddr::Alternative(false,  true, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1100, SlaveAddr::Alternative( true, false, false).addr(DEVICE_BASE_ADDRESS));
        assert_eq!(0b100_1111, SlaveAddr::Alternative( true,  true,  true).addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_convert_temperature() {
        assert_eq!(   0.0, convert_temp(0b0000_0000, 0b0101_1010));
        assert_eq!(   0.5, convert_temp(0b0000_0000, 0b1101_1010));
        assert_eq!(   1.0, convert_temp(0b0000_0001, 0b0101_1010));
        assert_eq!(   2.0, convert_temp(0b0000_0010, 0b0101_1010));
        assert_eq!(   4.0, convert_temp(0b0000_0100, 0b0101_1010));
        assert_eq!(   8.0, convert_temp(0b0000_1000, 0b0101_1010));
        assert_eq!(  16.0, convert_temp(0b0001_0000, 0b0101_1010));
        assert_eq!(  32.0, convert_temp(0b0010_0000, 0b0101_1010));
        assert_eq!(  64.0, convert_temp(0b0100_0000, 0b0101_1010));
        assert_eq!( 127.5, convert_temp(0b0111_1111, 0b1101_1010));
        assert_eq!(  -0.0, convert_temp(0b1000_0000, 0b0101_1010));
        assert_eq!(  -0.5, convert_temp(0b1000_0000, 0b1101_1010));
        assert_eq!(  -1.0, convert_temp(0b1000_0001, 0b0101_1010));
        assert_eq!(  -2.0, convert_temp(0b1000_0010, 0b0101_1010));
        assert_eq!(  -4.0, convert_temp(0b1000_0100, 0b0101_1010));
        assert_eq!(  -8.0, convert_temp(0b1000_1000, 0b0101_1010));
        assert_eq!( -16.0, convert_temp(0b1001_0000, 0b0101_1010));
        assert_eq!( -32.0, convert_temp(0b1010_0000, 0b0101_1010));
        assert_eq!( -64.0, convert_temp(0b1100_0000, 0b0101_1010));
        assert_eq!(-127.5, convert_temp(0b1111_1111, 0b1101_1010));
    }
}