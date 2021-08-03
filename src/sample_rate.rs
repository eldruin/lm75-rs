use crate::{marker, private, Error};
use crate::device_impl::BitFlags;

#[doc(hidden)]
pub trait SampleRateSupport<E>: private::Sealed {
    fn check_period_is_appropriate(value: u16) -> Result<(), Error<E>>;
    fn convert_sample_rate_to_register(period: u16) -> u8;
    fn convert_sample_rate_from_register(byte: u8) -> u16;
}

impl<E> SampleRateSupport<E> for marker::TemperatureIdleRegister {
    fn check_period_is_appropriate(value: u16) -> Result<(), Error<E>> {
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