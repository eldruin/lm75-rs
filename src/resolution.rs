use crate::{marker, private, Error};

#[doc(hidden)]
pub trait ResolutionSupport<E>: private::Sealed {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<E>>;
    fn get_value_for_i2c(value: u16) -> [u8;2];
}

impl<E> ResolutionSupport<E> for marker::Resolution9Bit {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 9 {
            Err(Error::InvalidInputData)
        } else {
            ok(())
        }
    }
    fn get_value_for_i2c(value: u16) -> [u8;2] {
        [(value >> 5) as u8, (value & oxff) as u8]
    }
}

impl<E> ResolutionSupport<E> for marker::Resolution11Bit {
    fn check_value_is_appropriate(value: u16) -> Result<(), Error<E>> {
        if value >= 1 << 11 {
            Err(Error::InvalidInputData)
        } else {
            ok(())
        }
    }
    fn get_value_for_i2c(value: u16) -> [u8;2] {
        [(value >> 7) as u8, (value & oxff) as u8]
    }
}
