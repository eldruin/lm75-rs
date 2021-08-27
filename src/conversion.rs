//! Value conversions
use crate::markers::BitMasks;

pub fn convert_temp_from_register(msb: u8, lsb: u8, mask: u16) -> f32 {
    // msb is stored as two's complement
    let msb = f32::from(msb as i8);
    let decimal = f32::from((lsb & mask as u8) >> 5) * 0.125;
    msb + decimal
}

pub fn convert_temp_to_register(temp: f32, mask: u16) -> (u8, u8) {
    let int = (temp / 0.125) as i16 as u16;
    let binary = int << 5;
    let msb = (binary >> 8) as u8;
    let lsb = (binary & mask) as u8;
    (msb, lsb)
}

pub fn convert_sample_rate_from_register(byte: u8) -> u16 {
    // Bits [4:0] hold sample rate value
    u16::from(byte & BitMasks::SAMPLE_RATE_MASK) * 100
}

pub fn convert_sample_rate_to_register(period: u16) -> u8 {
    // Bits [4:0] hold sample rate value
    (period / 100) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markers::BitMasks;

    macro_rules! assert_near {
        ($a:expr, $b:expr) => {
            assert!(($a + 0.01) > $b);
            assert!(($a - 0.01) < $b);
        };
    }

    #[test]
    fn can_convert_temperature_from_register() {
        assert_near!(
            convert_temp_from_register(0b0111_1101, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            125.0
        );
        assert_near!(
            convert_temp_from_register(0b0001_1001, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            25.0
        );
        assert_near!(
            convert_temp_from_register(0b1110_0111, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            -25.0
        );
        assert_near!(
            convert_temp_from_register(0b1100_1001, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            -55.0
        );
        assert_near!(
            convert_temp_from_register(0b0000_0000, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            0.0
        );
        assert_near!(
            convert_temp_from_register(0b0000_0000, 0b1101_1010, BitMasks::RESOLUTION_9BIT),
            0.5
        );
        assert_near!(
            convert_temp_from_register(0b0010_0000, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            32.0
        );
        assert_near!(
            convert_temp_from_register(0b0100_1011, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            75.0
        );
        assert_near!(
            convert_temp_from_register(0b0101_0000, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            80.0
        );
        assert_near!(
            convert_temp_from_register(0b0111_1111, 0b1101_1010, BitMasks::RESOLUTION_9BIT),
            127.5
        );
        assert_near!(
            convert_temp_from_register(0b1111_1111, 0b1101_1010, BitMasks::RESOLUTION_9BIT),
            -0.5
        );
        assert_near!(
            convert_temp_from_register(0b1111_1111, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            -1.0
        );
        assert_near!(
            convert_temp_from_register(0b1111_1101, 0b1101_1010, BitMasks::RESOLUTION_9BIT),
            -2.5
        );
        assert_near!(
            convert_temp_from_register(0b1110_0000, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            -32.0
        );
        assert_near!(
            convert_temp_from_register(0b1000_0000, 0b1101_1010, BitMasks::RESOLUTION_9BIT),
            -127.5
        );
        assert_near!(
            convert_temp_from_register(0b1000_0000, 0b0101_1010, BitMasks::RESOLUTION_9BIT),
            -128.0
        );

        assert_near!(
            convert_temp_from_register(0b0111_1101, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            125.250
        );
        assert_near!(
            convert_temp_from_register(0b0001_1001, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            25.250
        );
        assert_near!(
            convert_temp_from_register(0b1110_0111, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            -24.750
        );
        assert_near!(
            convert_temp_from_register(0b1100_1001, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            -54.750
        );
        assert_near!(
            convert_temp_from_register(0b0000_0000, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            0.250
        );
        assert_near!(
            convert_temp_from_register(0b0000_0000, 0b1101_1010, BitMasks::RESOLUTION_11BIT),
            0.750
        );
        assert_near!(
            convert_temp_from_register(0b0010_0000, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            32.250
        );
        assert_near!(
            convert_temp_from_register(0b0100_1011, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            75.250
        );
        assert_near!(
            convert_temp_from_register(0b0101_0000, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            80.250
        );
        assert_near!(
            convert_temp_from_register(0b0111_1111, 0b1101_1010, BitMasks::RESOLUTION_11BIT),
            127.750
        );
        assert_near!(
            convert_temp_from_register(0b1111_1111, 0b1101_1010, BitMasks::RESOLUTION_11BIT),
            -0.250
        );
        assert_near!(
            convert_temp_from_register(0b1111_1111, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            -0.750
        );
        assert_near!(
            convert_temp_from_register(0b1111_1101, 0b1101_1010, BitMasks::RESOLUTION_11BIT),
            -2.250
        );
        assert_near!(
            convert_temp_from_register(0b1110_0000, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            -31.750
        );
        assert_near!(
            convert_temp_from_register(0b1000_0000, 0b1101_1010, BitMasks::RESOLUTION_11BIT),
            -127.250
        );
        assert_near!(
            convert_temp_from_register(0b1000_0000, 0b0101_1010, BitMasks::RESOLUTION_11BIT),
            -127.750
        );
    }

    #[test]
    fn can_convert_temperature_to_register() {
        assert_eq!(
            (0b0000_0010, 0b0000_0000),
            convert_temp_to_register(2.4, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b0000_0010, 0b1000_0000),
            convert_temp_to_register(2.6, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b1111_1110, 0b0000_0000),
            convert_temp_to_register(-2.0, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b1111_1101, 0b1000_0000),
            convert_temp_to_register(-2.6, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b0111_1101, 0b0000_0000),
            convert_temp_to_register(125.0, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b0001_1001, 0b0000_0000),
            convert_temp_to_register(25.0, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b1110_0111, 0b0000_0000),
            convert_temp_to_register(-25.0, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b1100_1001, 0b0000_0000),
            convert_temp_to_register(-55.0, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b0000_0000, 0b0000_0000),
            convert_temp_to_register(0.0, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b0000_0000, 0b1000_0000),
            convert_temp_to_register(0.5, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b0010_0000, 0b0000_0000),
            convert_temp_to_register(32.0, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b0111_1111, 0b1000_0000),
            convert_temp_to_register(127.5, BitMasks::RESOLUTION_9BIT)
        );
        assert_eq!(
            (0b1000_0000, 0b0000_0000),
            convert_temp_to_register(-128.0, BitMasks::RESOLUTION_9BIT)
        );

        assert_eq!(
            (0b0000_0010, 0b0110_0000),
            convert_temp_to_register(2.4, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b0000_0010, 0b1000_0000),
            convert_temp_to_register(2.6, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b1111_1110, 0b0000_0000),
            convert_temp_to_register(-2.0, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b1111_1101, 0b1000_0000),
            convert_temp_to_register(-2.6, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b0111_1101, 0b0000_0000),
            convert_temp_to_register(125.0, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b0001_1001, 0b0000_0000),
            convert_temp_to_register(25.0, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b1110_0111, 0b0000_0000),
            convert_temp_to_register(-25.0, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b1100_1001, 0b0000_0000),
            convert_temp_to_register(-55.0, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b0000_0000, 0b0000_0000),
            convert_temp_to_register(0.0, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b0000_0000, 0b1000_0000),
            convert_temp_to_register(0.5, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b0010_0000, 0b0000_0000),
            convert_temp_to_register(32.0, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b0111_1111, 0b1000_0000),
            convert_temp_to_register(127.5, BitMasks::RESOLUTION_11BIT)
        );
        assert_eq!(
            (0b1000_0000, 0b0000_0000),
            convert_temp_to_register(-128.0, BitMasks::RESOLUTION_11BIT)
        );
    }

    #[test]
    fn can_convert_sample_rate_from_register() {
        assert_eq!(convert_sample_rate_from_register(0b0001_1111), 3100);
        assert_eq!(convert_sample_rate_from_register(0b1111_0000), 1600);
        assert_eq!(convert_sample_rate_from_register(0b0000_0001), 100);
    }

    #[test]
    fn can_convert_sample_rate_to_register() {
        assert_eq!(convert_sample_rate_to_register(3100), 0b0001_1111);
        assert_eq!(convert_sample_rate_to_register(1600), 0b0001_0000);
        assert_eq!(convert_sample_rate_to_register(100), 0b0000_0001);
    }
}
