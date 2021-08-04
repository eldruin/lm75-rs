//! Value conversions

pub fn convert_temp_from_register(msb: u8, lsb: u8) -> f32 {
    // msb is stored as two's complement
    let msb = msb as i8 as f32;
    let decimal = ((lsb & mask as u8) >> 5) as f32 * 0.125;
    msb + decimal
}

pub fn convert_temp_to_register(temp: f32) -> (u8, u8) {
    let int = (temp / 0.125) as i16 as u16;
    let binary = int << 5;
    let msb = (binary >> 8) as u8;
    let lsb = (binary & 0xFF) as u8;
    (msb, lsb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Resolution::*;

    #[test]
    fn can_convert_temperature_from_register() {
        assert_eq!(convert_temp_from_register(0b0111_1101, 0b0101_1010), 125.0);
        assert_eq!(convert_temp_from_register(0b0001_1001, 0b0101_1010), 25.0);
        assert_eq!(convert_temp_from_register(0b1110_0111, 0b0101_1010), -25.0);
        assert_eq!(convert_temp_from_register(0b1100_1001, 0b0101_1010), -55.0);
        assert_eq!(convert_temp_from_register(0b0000_0000, 0b0101_1010), 0.0);
        assert_eq!(convert_temp_from_register(0b0000_0000, 0b1101_1010), 0.5);
        assert_eq!(convert_temp_from_register(0b0010_0000, 0b0101_1010), 32.0);
        assert_eq!(convert_temp_from_register(0b0100_1011, 0b0101_1010), 75.0);
        assert_eq!(convert_temp_from_register(0b0101_0000, 0b0101_1010), 80.0);
        assert_eq!(convert_temp_from_register(0b0111_1111, 0b1101_1010), 127.5);
        assert_eq!(convert_temp_from_register(0b1111_1111, 0b1101_1010), -0.5);
        assert_eq!(convert_temp_from_register(0b1111_1111, 0b0101_1010), -1.0);
        assert_eq!(convert_temp_from_register(0b1111_1101, 0b1101_1010), -2.5);
        assert_eq!(convert_temp_from_register(0b1110_0000, 0b0101_1010), -32.0);
        assert_eq!(convert_temp_from_register(0b1000_0000, 0b1101_1010), -127.5);
        assert_eq!(convert_temp_from_register(0b1000_0000, 0b0101_1010), -128.0);

        assert_eq!(convert_temp_from_register(0b0111_1101, 0b0101_1010), 125.250);
        assert_eq!(convert_temp_from_register(0b0001_1001, 0b0101_1010), 25.250);
        assert_eq!(convert_temp_from_register(0b1110_0111, 0b0101_1010), -24.750);
        assert_eq!(convert_temp_from_register(0b1100_1001, 0b0101_1010), -54.750);
        assert_eq!(convert_temp_from_register(0b0000_0000, 0b0101_1010), 0.250);
        assert_eq!(convert_temp_from_register(0b0000_0000, 0b1101_1010), 0.750);
        assert_eq!(convert_temp_from_register(0b0010_0000, 0b0101_1010), 32.250);
        assert_eq!(convert_temp_from_register(0b0100_1011, 0b0101_1010), 75.250);
        assert_eq!(convert_temp_from_register(0b0101_0000, 0b0101_1010), 80.250);
        assert_eq!(convert_temp_from_register(0b0111_1111, 0b1101_1010), 127.750);
        assert_eq!(convert_temp_from_register(0b1111_1111, 0b1101_1010), -0.250);
        assert_eq!(convert_temp_from_register(0b1111_1111, 0b0101_1010), -0.750);
        assert_eq!(convert_temp_from_register(0b1111_1101, 0b1101_1010), -2.250);
        assert_eq!(convert_temp_from_register(0b1110_0000, 0b0101_1010), -31.750);
        assert_eq!(convert_temp_from_register(0b1000_0000, 0b1101_1010), -127.250);
        assert_eq!(convert_temp_from_register(0b1000_0000, 0b0101_1010), -127.750);
    }

    #[test]
    fn can_convert_temperature_to_register() {
        assert_eq!((0b0000_0010, 0b0000_0000), convert_temp_to_register(2.4));
        assert_eq!((0b0000_0010, 0b1000_0000), convert_temp_to_register(2.6));
        assert_eq!((0b1111_1110, 0b0000_0000), convert_temp_to_register(-2.0));
        assert_eq!((0b1111_1101, 0b1000_0000), convert_temp_to_register(-2.6));
        assert_eq!((0b0111_1101, 0b0000_0000), convert_temp_to_register(125.0));
        assert_eq!((0b0001_1001, 0b0000_0000), convert_temp_to_register(25.0));
        assert_eq!((0b1110_0111, 0b0000_0000), convert_temp_to_register(-25.0));
        assert_eq!((0b1100_1001, 0b0000_0000), convert_temp_to_register(-55.0));
        assert_eq!((0b0000_0000, 0b0000_0000), convert_temp_to_register(0.0));
        assert_eq!((0b0000_0000, 0b1000_0000), convert_temp_to_register(0.5));
        assert_eq!((0b0010_0000, 0b0000_0000), convert_temp_to_register(32.0));
        assert_eq!((0b0111_1111, 0b1000_0000), convert_temp_to_register(127.5));
        assert_eq!((0b1000_0000, 0b0000_0000), convert_temp_to_register(-128.0));

        assert_eq!((0b0000_0010, 0b0110_0000), convert_temp_to_register(2.4));
        assert_eq!((0b0000_0010, 0b1000_0000), convert_temp_to_register(2.6));
        assert_eq!((0b1111_1110, 0b0000_0000), convert_temp_to_register(-2.0));
        assert_eq!((0b1111_1101, 0b1000_0000), convert_temp_to_register(-2.6));
        assert_eq!((0b0111_1101, 0b0000_0000), convert_temp_to_register(125.0));
        assert_eq!((0b0001_1001, 0b0000_0000), convert_temp_to_register(25.0));
        assert_eq!((0b1110_0111, 0b0000_0000), convert_temp_to_register(-25.0));
        assert_eq!((0b1100_1001, 0b0000_0000), convert_temp_to_register(-55.0));
        assert_eq!((0b0000_0000, 0b0000_0000), convert_temp_to_register(0.0));
        assert_eq!((0b0000_0000, 0b1000_0000), convert_temp_to_register(0.5));
        assert_eq!((0b0010_0000, 0b0000_0000), convert_temp_to_register(32.0));
        assert_eq!((0b0111_1111, 0b1000_0000), convert_temp_to_register(127.5));
        assert_eq!((0b1000_0000, 0b0000_0000), convert_temp_to_register(-128.0));
    }
}
