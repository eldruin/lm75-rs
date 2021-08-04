use crate::{ic, private, Error};
use crate::device_impl::BitFlags;

#[doc(hidden)]
pub trait SampleRateSupport<E>: private::Sealed {
    fn check_period_is_appropriate(value: u16) -> Result<(), Error<E>>;
    fn convert_sample_rate_to_register(period: u16) -> u8;
    fn convert_sample_rate_from_register(byte: u8) -> u16;
}

impl<E> SampleRateSupport<E> for ic::Pct2075 {
    fn check_period_is_appropriate(period: u16) -> Result<(), Error<E>> {
        if period > 3100 || period % 100 != 0 {
            return Err(Error::InvalidInputData);
        } else {
            Ok(())
        }
    }
    fn convert_sample_rate_to_register(period: u16) -> u8 {
        (period / 100) as u8
    }

    fn convert_sample_rate_from_register(byte: u8) -> u16 {
        // Bits [4:0] hold sample rate value
        ((byte as u16) & BitFlags::SAMPLE_RATE_MASK) * 100
    }
}

pub trait ResolutionSupport<E>: private::Sealed {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<E>>;
    fn get_value_for_i2c(value: u16) -> [u8; 2];
}

impl<E> ResolutionSupport<E> for ic::Pct2075 {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 9 {
            Err(Error::InvalidInputData)
        } else {
            Ok(())
        }
    }
    fn get_value_for_i2c(value: u16) -> [u8; 2] {
        [(value >> 5) as u8, (value & 0xff) as u8]
    }
}

impl<E> ResolutionSupport<E> for ic::Lm75 {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 11 {
            Err(Error::InvalidInputData)
        } else {
            Ok(())
        }
    }
    fn get_value_for_i2c(value: u16) -> [u8; 2] {
        [(value >> 7) as u8, (value & 0xff) as u8]
    }
}