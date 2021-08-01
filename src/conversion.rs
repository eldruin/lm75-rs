//! Value conversions
use crate::Resolution::*;
use crate::Resolution;

pub fn convert_temp_from_register(msb: u8, lsb: u8, mask:Resolution) -> f32 {
    // msb is stored as two's complement
    let msb = msb as i8 as f32;
    let decimal = (lsb >> 5) as f32 * 0.125;
    let temp = msb + decimal;
    temp & mask
}

pub fn convert_temp_to_register(temp: f32, mask: Resolution) -> (u8, u8) {
    let int = (temp / 0.125) as i16 as u16;
    let binary = int << 5;
    let bytes = binary.to_be_bytes();
    (bytes[0] & &mask, bytes[1] & &mask)
}

pub fn convert_sample_rate_from_register(byte: u8) -> u16 {
    // Bits [4:0] hold sample rate value
    ((byte as u16) & 0x1F) * 100
}

pub fn convert_sample_rate_to_register(period: u16) -> u8 {
    // Bits [4:0] hold sample rate value
    (period / 100) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_near {
        ($a:expr, $b:expr) => {
            assert!(($a + 0.1) > $b);
            assert!(($a - 0.1) < $b);
        };
    }

    #[test]
    fn can_convert_temperature_from_register() {
        assert_near!(convert_temp_from_register(0b0111_1101, 0b0101_1010), 125.0);
        assert_near!(convert_temp_from_register(0b0001_1001, 0b0101_1010), 25.0);
        assert_near!(convert_temp_from_register(0b1110_0111, 0b0101_1010), -25.0);
        assert_near!(convert_temp_from_register(0b1100_1001, 0b0101_1010), -55.0);

        assert_near!(convert_temp_from_register(0b0000_0000, 0b0101_1010), 0.0);
        assert_near!(convert_temp_from_register(0b0000_0000, 0b1101_1010), 0.5);
        assert_near!(convert_temp_from_register(0b0000_0001, 0b0101_1010), 1.0);
        assert_near!(convert_temp_from_register(0b0000_0010, 0b0101_1010), 2.0);
        assert_near!(convert_temp_from_register(0b0000_0100, 0b0101_1010), 4.0);
        assert_near!(convert_temp_from_register(0b0000_1000, 0b0101_1010), 8.0);
        assert_near!(convert_temp_from_register(0b0001_0000, 0b0101_1010), 16.0);
        assert_near!(convert_temp_from_register(0b0010_0000, 0b0101_1010), 32.0);
        assert_near!(convert_temp_from_register(0b0100_0000, 0b0101_1010), 64.0);
        assert_near!(convert_temp_from_register(0b0100_1011, 0b0101_1010), 75.0);
        assert_near!(convert_temp_from_register(0b0101_0000, 0b0101_1010), 80.0);
        assert_near!(convert_temp_from_register(0b0111_1111, 0b1101_1010), 127.5);

        assert_near!(convert_temp_from_register(0b1111_1111, 0b1101_1010), -0.5);
        assert_near!(convert_temp_from_register(0b1111_1111, 0b0101_1010), -1.0);
        assert_near!(convert_temp_from_register(0b1111_1110, 0b0101_1010), -2.0);
        assert_near!(convert_temp_from_register(0b1111_1101, 0b1101_1010), -2.5);
        assert_near!(convert_temp_from_register(0b1111_1100, 0b0101_1010), -4.0);
        assert_near!(convert_temp_from_register(0b1111_1000, 0b0101_1010), -8.0);
        assert_near!(convert_temp_from_register(0b1111_0000, 0b0101_1010), -16.0);
        assert_near!(convert_temp_from_register(0b1110_0000, 0b0101_1010), -32.0);
        assert_near!(convert_temp_from_register(0b1100_0000, 0b0101_1010), -64.0);
        assert_near!(convert_temp_from_register(0b1000_0000, 0b1101_1010), -127.5);
        assert_near!(convert_temp_from_register(0b1000_0000, 0b0101_1010), -128.0);
    }

    #[test]
    fn can_convert_temperature_to_register() {
        assert_eq!((0b0000_0010, 0), convert_temp_to_register(2.4));
        assert_eq!((0b0000_0010, 1), convert_temp_to_register(2.6));
        assert_eq!((0b1111_1110, 0), convert_temp_to_register(-2.4));
        assert_eq!((0b1111_1101, 1), convert_temp_to_register(-2.6));
        assert_eq!((0b0111_1101, 0), convert_temp_to_register(125.0));
        assert_eq!((0b0001_1001, 0), convert_temp_to_register(25.0));
        assert_eq!((0b1110_0111, 0), convert_temp_to_register(-25.0));
        assert_eq!((0b1100_1001, 0), convert_temp_to_register(-55.0));

        assert_eq!((0b0000_0000, 0), convert_temp_to_register(0.0));
        assert_eq!((0b0000_0000, 1), convert_temp_to_register(0.5));
        assert_eq!((0b0000_0001, 0), convert_temp_to_register(1.0));
        assert_eq!((0b0000_0010, 0), convert_temp_to_register(2.0));
        assert_eq!((0b0000_0100, 0), convert_temp_to_register(4.0));
        assert_eq!((0b0000_1000, 0), convert_temp_to_register(8.0));
        assert_eq!((0b0001_0000, 0), convert_temp_to_register(16.0));
        assert_eq!((0b0010_0000, 0), convert_temp_to_register(32.0));
        assert_eq!((0b0100_0000, 0), convert_temp_to_register(64.0));
        assert_eq!((0b0100_1011, 0), convert_temp_to_register(75.0));
        assert_eq!((0b0101_0000, 0), convert_temp_to_register(80.0));
        assert_eq!((0b0111_1111, 1), convert_temp_to_register(127.5));

        assert_eq!((0b1111_1111, 1), convert_temp_to_register(-0.5));
        assert_eq!((0b1111_1111, 0), convert_temp_to_register(-1.0));
        assert_eq!((0b1111_1110, 0), convert_temp_to_register(-2.0));
        assert_eq!((0b1111_1100, 0), convert_temp_to_register(-4.0));
        assert_eq!((0b1111_1000, 0), convert_temp_to_register(-8.0));
        assert_eq!((0b1111_0000, 0), convert_temp_to_register(-16.0));
        assert_eq!((0b1110_0000, 0), convert_temp_to_register(-32.0));
        assert_eq!((0b1100_0000, 0), convert_temp_to_register(-64.0));
        assert_eq!((0b1000_0000, 1), convert_temp_to_register(-127.5));
        assert_eq!((0b1000_0000, 0), convert_temp_to_register(-128.0));
    }
}
