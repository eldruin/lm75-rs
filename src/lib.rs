//! This is a platform agnostic Rust driver for the LM75 temperature
//! sensor and thermal watchdog, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the device.
//! - Read the temperature.
//! - Set the fault queue.
//! - Set the OS temperature.
//! - Set the hysteresis temperature.
//! - Set the OS operation mode.
//! - Set the OS polarity.
//!
//! ## The device
//!
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
//! This driver is also compatible with LM75A, LM75B and LM75C: [LM75B/C Datasheet]
//!
//! [LM75B/C Datasheet]: http://www.ti.com/lit/ds/symlink/lm75b.pdf
//!
//! And also at least with the devices MAX7500, MAX6625, MAX6626, DS75LV,
//! and DS7505.
//!
//! ## Usage examples (see also examples folder)
//!
//! ### Read temperature
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let address = SlaveAddr::default();
//! let mut sensor = Lm75::new(dev, address);
//! let temperature = sensor.read_temperature().unwrap();
//! println!("Temperature: {}", temperature);
//! # }
//! ```
//!
//! ### Provide an alternative address
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let (a2, a1, a0) = (false, false, true);
//! let address = SlaveAddr::Alternative(a2, a1, a0);
//! let mut sensor = Lm75::new(dev, address);
//! # }
//! ```
//!
//! ### Set the fault queue
//!
//! This is the number of consecutive faults necessary to trigger
//! an OS condition.
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr, FaultQueue };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_fault_queue(FaultQueue::_4).unwrap();
//! # }
//! ```
//!
//! ### Set the OS polarity
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr, OsPolarity };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_os_polarity(OsPolarity::ActiveHigh).unwrap();
//! # }
//! ```
//!
//! ### Set the OS operation mode
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr, OsMode };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_os_mode(OsMode::Interrupt).unwrap();
//! # }
//! ```
//!
//! ### Set the OS temperature
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_os_temperature(50.0).unwrap();
//! # }
//! ```
//!
//! ### Set the hysteresis temperature
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.set_hysteresis_temperature(40.0).unwrap();
//! # }
//! ```
//!
//! ### Enable / disable the sensor
//!
//! ```no_run
//! extern crate linux_embedded_hal as hal;
//! extern crate lm75;
//!
//! use hal::I2cdev;
//! use lm75::{ Lm75, SlaveAddr };
//!
//! # fn main() {
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lm75::new(dev, SlaveAddr::default());
//! sensor.disable().unwrap(); // shutdown
//! sensor.enable().unwrap();
//! # }
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

extern crate embedded_hal as hal;
use hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data
    InvalidInputData,
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

/// Fault queue
///
/// Number of consecutive faults necessary to trigger OS condition.
#[derive(Debug, Clone)]
pub enum FaultQueue {
    /// 1 fault will trigger OS condition
    _1,
    /// 2 consecutive faults will trigger OS condition
    _2,
    /// 4 consecutive faults will trigger OS condition
    _4,
    /// 6 consecutive faults will trigger OS condition
    _6,
}

/// OS polarity
#[derive(Debug, Clone)]
pub enum OsPolarity {
    /// Active low
    ActiveLow,
    /// Active high
    ActiveHigh
}

/// OS operation mode
#[derive(Debug, Clone)]
pub enum OsMode {
    /// Comparator
    Comparator,
    /// Interrupt
    Interrupt
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
    const SHUTDOWN     : u8 = 0b0000_0001;
    const COMP_INT     : u8 = 0b0000_0010;
    const OS_POLARITY  : u8 = 0b0000_0100;
    const FAULT_QUEUE0 : u8 = 0b0000_1000;
    const FAULT_QUEUE1 : u8 = 0b0001_0000;
}

/// LM75 device driver.
#[derive(Debug, Default)]
pub struct Lm75<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: u8,
}

mod conversion;

impl<I2C, E> Lm75<I2C>
where
    I2C: i2c::Write<Error = E>
{
    /// Create new instance of the LM75 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Lm75 {
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
        let config = self.config;
        self.write_config(config & !BitFlags::SHUTDOWN)
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config | BitFlags::SHUTDOWN)
    }

    /// Set the fault queue.
    ///
    /// Set the number of consecutive faults that will trigger an OS condition.
    pub fn set_fault_queue(&mut self, fq: FaultQueue) -> Result<(), Error<E>> {
        let mut config = self.config;
        match fq {
            FaultQueue::_1 => config = config & !BitFlags::FAULT_QUEUE1 & !BitFlags::FAULT_QUEUE0,
            FaultQueue::_2 => config = config & !BitFlags::FAULT_QUEUE1 |  BitFlags::FAULT_QUEUE0,
            FaultQueue::_4 => config = config |  BitFlags::FAULT_QUEUE1 & !BitFlags::FAULT_QUEUE0,
            FaultQueue::_6 => config = config |  BitFlags::FAULT_QUEUE1 |  BitFlags::FAULT_QUEUE0,
        }
        self.write_config(config)
    }

    /// Set the OS polarity.
    pub fn set_os_polarity(&mut self, polarity: OsPolarity) -> Result<(), Error<E>> {
        let mut config = self.config;
        match polarity {
            OsPolarity::ActiveLow  => config = config & !BitFlags::OS_POLARITY,
            OsPolarity::ActiveHigh => config = config |  BitFlags::OS_POLARITY,
        }
        self.write_config(config)
    }

    /// Set the OS operation mode.
    pub fn set_os_mode(&mut self, mode: OsMode) -> Result<(), Error<E>> {
        let mut config = self.config;
        match mode {
            OsMode::Comparator => config = config & !BitFlags::COMP_INT,
            OsMode::Interrupt  => config = config |  BitFlags::COMP_INT,
        }
        self.write_config(config)
    }

    /// Set the OS temperature.
    pub fn set_os_temperature(&mut self, temperature: f32) -> Result<(), Error<E>> {
        if temperature < -55.0 || temperature > 125.0 {
            return Err(Error::InvalidInputData);
        }
        let (msb, lsb) = conversion::convert_temp_to_register(temperature);
        self.i2c
            .write(self.address, &[Register::T_OS, msb, lsb])
            .map_err(Error::I2C)
    }

    /// Set the hysteresis temperature.
    pub fn set_hysteresis_temperature(&mut self, temperature: f32) -> Result<(), Error<E>> {
        if temperature < -55.0 || temperature > 125.0 {
            return Err(Error::InvalidInputData);
        }
        let (msb, lsb) = conversion::convert_temp_to_register(temperature);
        self.i2c
            .write(self.address, &[Register::T_HYST, msb, lsb])
            .map_err(Error::I2C)
    }

    fn write_config(&mut self, config: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIGURATION, config])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}

impl<I2C, E> Lm75<I2C>
where
    I2C: i2c::WriteRead<Error = E>
{
    /// Read the temperature from the sensor.
    pub fn read_temperature(&mut self) -> Result<f32, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[Register::TEMPERATURE], &mut data)
            .map_err(Error::I2C)?;
        Ok(conversion::convert_temp_from_register(data[0], data[1]))
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
}