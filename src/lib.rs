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
//! This driver is also compatible with at least [LM75A], [LM75B, LM75C],
//! [AT30TS75A], [DS1775], [DS75], [DS7505], [G751], [MAX7500/1/2/3/4],
//! [MAX6625], [MCP9800/1/2/3], [STDS75], [TCN75].
//!
//! [AT30TS75A]: http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-8839-DTS-AT30TS75A-Datasheet.pdf
//! [DS1775]: https://datasheets.maximintegrated.com/en/ds/DS1775-DS1775R.pdf
//! [DS75]: https://datasheets.maximintegrated.com/en/ds/DS75.pdf
//! [DS7505]: https://datasheets.maximintegrated.com/en/ds/DS7505.pdf
//! [G751]: http://www.gmt.com.tw/product/datasheet/EDS-751.pdf
//! [LM75A]: https://www.nxp.com/docs/en/data-sheet/LM75A.pdf
//! [LM75B, LM75C]: http://www.ti.com/lit/ds/symlink/lm75b.pdf
//! [MAX6625]: https://datasheets.maximintegrated.com/en/ds/MAX6625-MAX6626.pdf
//! [MAX7500/1/2/3/4]: https://datasheets.maximintegrated.com/en/ds/MAX7500-MAX7504.pdf
//! [MCP9800/1/2/3]: http://ww1.microchip.com/downloads/en/DeviceDoc/21909d.pdf
//! [STDS75]: https://www.st.com/resource/en/datasheet/stds75.pdf
//! [TCN75]: http://ww1.microchip.com/downloads/en/DeviceDoc/21490D.pdf
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read temperature
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
//! let temp_celsius = sensor.read_temperature().unwrap();
//! println!("Temperature: {}ºC", temp_celsius);
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
//! let temp_celsius = 50.0;
//! sensor.set_os_temperature(temp_celsius).unwrap();
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
//! let temp_celsius = 40.0;
//! sensor.set_hysteresis_temperature(temp_celsius).unwrap();
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

#![deny(missing_docs, unsafe_code)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaultQueue {
    /// 1 fault will trigger OS condition (default)
    _1,
    /// 2 consecutive faults will trigger OS condition
    _2,
    /// 4 consecutive faults will trigger OS condition
    _4,
    /// 6 consecutive faults will trigger OS condition
    _6,
}

/// OS polarity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OsPolarity {
    /// Active low (default)
    ActiveLow,
    /// Active high
    ActiveHigh
}

/// OS operation mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OsMode {
    /// Comparator (default)
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

#[derive(Debug, Clone, Copy)]
struct Config {
    bits: u8,
}

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0 }
    }
}

/// LM75 device driver.
#[derive(Debug, Default)]
pub struct Lm75<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// The I²C device address.
    address: u8,
    /// Configuration register status.
    config: Config,
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
            config: Config::default()
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the sensor (default state).
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_low(BitFlags::SHUTDOWN))
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config;
        self.write_config(config.with_high(BitFlags::SHUTDOWN))
    }

    /// Set the fault queue.
    ///
    /// Set the number of consecutive faults that will trigger an OS condition.
    pub fn set_fault_queue(&mut self, fq: FaultQueue) -> Result<(), Error<E>> {
        let config = self.config;
        match fq {
            FaultQueue::_1 => self.write_config(config.with_low( BitFlags::FAULT_QUEUE1).with_low( BitFlags::FAULT_QUEUE0)),
            FaultQueue::_2 => self.write_config(config.with_low( BitFlags::FAULT_QUEUE1).with_high(BitFlags::FAULT_QUEUE0)),
            FaultQueue::_4 => self.write_config(config.with_high(BitFlags::FAULT_QUEUE1).with_low( BitFlags::FAULT_QUEUE0)),
            FaultQueue::_6 => self.write_config(config.with_high(BitFlags::FAULT_QUEUE1).with_high(BitFlags::FAULT_QUEUE0)),
        }
    }

    /// Set the OS polarity.
    pub fn set_os_polarity(&mut self, polarity: OsPolarity) -> Result<(), Error<E>> {
        let config = self.config;
        match polarity {
            OsPolarity::ActiveLow  => self.write_config(config.with_low( BitFlags::OS_POLARITY)),
            OsPolarity::ActiveHigh => self.write_config(config.with_high(BitFlags::OS_POLARITY)),
        }
    }

    /// Set the OS operation mode.
    pub fn set_os_mode(&mut self, mode: OsMode) -> Result<(), Error<E>> {
        let config = self.config;
        match mode {
            OsMode::Comparator => self.write_config(config.with_low( BitFlags::COMP_INT)),
            OsMode::Interrupt  => self.write_config(config.with_high(BitFlags::COMP_INT)),
        }
    }

    /// Set the OS temperature (celsius).
    pub fn set_os_temperature(&mut self, temperature: f32) -> Result<(), Error<E>> {
        if temperature < -55.0 || temperature > 125.0 {
            return Err(Error::InvalidInputData);
        }
        let (msb, lsb) = conversion::convert_temp_to_register(temperature);
        self.i2c
            .write(self.address, &[Register::T_OS, msb, lsb])
            .map_err(Error::I2C)
    }

    /// Set the hysteresis temperature (celsius).
    pub fn set_hysteresis_temperature(&mut self, temperature: f32) -> Result<(), Error<E>> {
        if temperature < -55.0 || temperature > 125.0 {
            return Err(Error::InvalidInputData);
        }
        let (msb, lsb) = conversion::convert_temp_to_register(temperature);
        self.i2c
            .write(self.address, &[Register::T_HYST, msb, lsb])
            .map_err(Error::I2C)
    }

    fn write_config(&mut self, config: Config) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::CONFIGURATION, config.bits])
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }
}

impl<I2C, E> Lm75<I2C>
where
    I2C: i2c::WriteRead<Error = E>
{
    /// Read the temperature from the sensor (celsius).
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