#![deny(unsafe_code)]

pub fn convert_temp_from_register(msb: u8, lsb: u8) -> f32 {
    // msb is stored as two's complement so we can just do this:
    let value = msb as i8;
    if (lsb >> 7) != 0 {
        value as f32 + 0.5
    }
    else {
        value as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_temperature_from_register() {
        assert_eq!( 125.0, convert_temp_from_register(0b0111_1101, 0b0101_1010));
        assert_eq!(  25.0, convert_temp_from_register(0b0001_1001, 0b0101_1010));
        assert_eq!( -25.0, convert_temp_from_register(0b1110_0111, 0b0101_1010));
        assert_eq!( -55.0, convert_temp_from_register(0b1100_1001, 0b0101_1010));

        assert_eq!(   0.0, convert_temp_from_register(0b0000_0000, 0b0101_1010));
        assert_eq!(   0.5, convert_temp_from_register(0b0000_0000, 0b1101_1010));
        assert_eq!(   1.0, convert_temp_from_register(0b0000_0001, 0b0101_1010));
        assert_eq!(   2.0, convert_temp_from_register(0b0000_0010, 0b0101_1010));
        assert_eq!(   4.0, convert_temp_from_register(0b0000_0100, 0b0101_1010));
        assert_eq!(   8.0, convert_temp_from_register(0b0000_1000, 0b0101_1010));
        assert_eq!(  16.0, convert_temp_from_register(0b0001_0000, 0b0101_1010));
        assert_eq!(  32.0, convert_temp_from_register(0b0010_0000, 0b0101_1010));
        assert_eq!(  64.0, convert_temp_from_register(0b0100_0000, 0b0101_1010));
        assert_eq!(  75.0, convert_temp_from_register(0b0100_1011, 0b0101_1010));
        assert_eq!(  80.0, convert_temp_from_register(0b0101_0000, 0b0101_1010));
        assert_eq!( 127.5, convert_temp_from_register(0b0111_1111, 0b1101_1010));

        assert_eq!(  -0.5, convert_temp_from_register(0b1111_1111, 0b1101_1010));
        assert_eq!(  -1.0, convert_temp_from_register(0b1111_1111, 0b0101_1010));
        assert_eq!(  -2.0, convert_temp_from_register(0b1111_1110, 0b0101_1010));
        assert_eq!(  -2.5, convert_temp_from_register(0b1111_1101, 0b1101_1010));
        assert_eq!(  -4.0, convert_temp_from_register(0b1111_1100, 0b0101_1010));
        assert_eq!(  -8.0, convert_temp_from_register(0b1111_1000, 0b0101_1010));
        assert_eq!( -16.0, convert_temp_from_register(0b1111_0000, 0b0101_1010));
        assert_eq!( -32.0, convert_temp_from_register(0b1110_0000, 0b0101_1010));
        assert_eq!( -64.0, convert_temp_from_register(0b1100_0000, 0b0101_1010));
        assert_eq!(-127.5, convert_temp_from_register(0b1000_0000, 0b1101_1010));
        assert_eq!(-128.0, convert_temp_from_register(0b1000_0000, 0b0101_1010));
    }
}