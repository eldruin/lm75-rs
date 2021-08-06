use crate::{ic, private};

pub struct BitMasks;

impl BitMasks {
    pub const RESOLUTION_9BIT: u16 = 0b1111_1111_1000_0000;
    pub const RESOLUTION_11BIT: u16 = 0b1111_1111_1110_0000;
    pub const SAMPLE_RATE_MASK: u8 = 0b0001_1111;
}

#[doc(hidden)]
pub trait ResolutionSupport<E>: private::Sealed {
    fn get_resolution_mask() -> u16;
}

impl<E> ResolutionSupport<E> for ic::Pct2075 {
    fn get_resolution_mask() -> u16 {
        BitMasks::RESOLUTION_11BIT
    }
}

impl<E> ResolutionSupport<E> for ic::Lm75 {
    fn get_resolution_mask() -> u16 {
        BitMasks::RESOLUTION_9BIT
    }
}
