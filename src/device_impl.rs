use crate::{
    conversion, Config, Error, FaultQueue, Lm75, OsMode, OsPolarity, SlaveAddr, DEVICE_BASE_ADDRESS,
};
use embedded_hal::blocking::i2c;

struct Register;
impl Register {
    const TEMPERATURE: u8 = 0x00;
    const CONFIGURATION: u8 = 0x01;
    const T_HYST: u8 = 0x02;
    const T_OS: u8 = 0x03;
}

struct BitFlags;
impl BitFlags {
    const SHUTDOWN: u8 = 0b0000_0001;
    const COMP_INT: u8 = 0b0000_0010;
    const OS_POLARITY: u8 = 0b0000_0100;
    const FAULT_QUEUE0: u8 = 0b0000_1000;
    const FAULT_QUEUE1: u8 = 0b0001_0000;
}

impl<I2C, E> Lm75<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the LM75 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Lm75 {
            i2c,
            address: address.addr(DEVICE_BASE_ADDRESS),
            config: Config::default(),
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
            FaultQueue::_1 => self.write_config(
                config
                    .with_low(BitFlags::FAULT_QUEUE1)
                    .with_low(BitFlags::FAULT_QUEUE0),
            ),
            FaultQueue::_2 => self.write_config(
                config
                    .with_low(BitFlags::FAULT_QUEUE1)
                    .with_high(BitFlags::FAULT_QUEUE0),
            ),
            FaultQueue::_4 => self.write_config(
                config
                    .with_high(BitFlags::FAULT_QUEUE1)
                    .with_low(BitFlags::FAULT_QUEUE0),
            ),
            FaultQueue::_6 => self.write_config(
                config
                    .with_high(BitFlags::FAULT_QUEUE1)
                    .with_high(BitFlags::FAULT_QUEUE0),
            ),
        }
    }

    /// Set the OS polarity.
    pub fn set_os_polarity(&mut self, polarity: OsPolarity) -> Result<(), Error<E>> {
        let config = self.config;
        match polarity {
            OsPolarity::ActiveLow => self.write_config(config.with_low(BitFlags::OS_POLARITY)),
            OsPolarity::ActiveHigh => self.write_config(config.with_high(BitFlags::OS_POLARITY)),
        }
    }

    /// Set the OS operation mode.
    pub fn set_os_mode(&mut self, mode: OsMode) -> Result<(), Error<E>> {
        let config = self.config;
        match mode {
            OsMode::Comparator => self.write_config(config.with_low(BitFlags::COMP_INT)),
            OsMode::Interrupt => self.write_config(config.with_high(BitFlags::COMP_INT)),
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
    I2C: i2c::WriteRead<Error = E>,
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
