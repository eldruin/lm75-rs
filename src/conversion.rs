#![deny(unsafe_code)]

pub fn convert_temp_from_register(msb: u8, lsb: u8) -> f32 {
    // msb is stored as two's complement so we can just do this:
    let value = msb as i8;
    if (lsb >> 7) != 0 {
        f32::from(value) + 0.5
    }
    else {
        f32::from(value)
    }
}

pub fn convert_temp_to_register(temp: f32) -> (u8, u8) {
    let msb = temp as i8 as u8;
    if temp < 0.0 {
        // abs() is not available for bare metal targets at the moment
        let diff = temp - f32::from(temp as i8);
        if diff > 0.499 || diff < -0.499 {
            if msb == 0 {
                // -0.5 case
                return (255, 1);
            }
            else {
                return (msb - 1, 1);
            }
        }
        else {
            return (msb, 0);
        }
    }
    if temp - f32::from(temp as i8) > 0.499 {
        (msb as u8, 1)
    }
    else {
        (msb as u8, 0)
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

    #[test]
    fn can_convert_temperature_to_register() {
        assert_eq!(convert_temp_to_register(   2.4), (0b0000_0010, 0));
        assert_eq!(convert_temp_to_register(   2.6), (0b0000_0010, 1));
        assert_eq!(convert_temp_to_register(  -2.4), (0b1111_1110, 0));
        assert_eq!(convert_temp_to_register(  -2.6), (0b1111_1101, 1));
        assert_eq!(convert_temp_to_register( 125.0), (0b0111_1101, 0));
        assert_eq!(convert_temp_to_register(  25.0), (0b0001_1001, 0));
        assert_eq!(convert_temp_to_register( -25.0), (0b1110_0111, 0));
        assert_eq!(convert_temp_to_register( -55.0), (0b1100_1001, 0));

        assert_eq!(convert_temp_to_register(   0.0), (0b0000_0000, 0));
        assert_eq!(convert_temp_to_register(   0.5), (0b0000_0000, 1));
        assert_eq!(convert_temp_to_register(   1.0), (0b0000_0001, 0));
        assert_eq!(convert_temp_to_register(   2.0), (0b0000_0010, 0));
        assert_eq!(convert_temp_to_register(   4.0), (0b0000_0100, 0));
        assert_eq!(convert_temp_to_register(   8.0), (0b0000_1000, 0));
        assert_eq!(convert_temp_to_register(  16.0), (0b0001_0000, 0));
        assert_eq!(convert_temp_to_register(  32.0), (0b0010_0000, 0));
        assert_eq!(convert_temp_to_register(  64.0), (0b0100_0000, 0));
        assert_eq!(convert_temp_to_register(  75.0), (0b0100_1011, 0));
        assert_eq!(convert_temp_to_register(  80.0), (0b0101_0000, 0));
        assert_eq!(convert_temp_to_register( 127.5), (0b0111_1111, 1));

        assert_eq!(convert_temp_to_register(  -0.5), (0b1111_1111, 1));
        assert_eq!(convert_temp_to_register(  -1.0), (0b1111_1111, 0));
        assert_eq!(convert_temp_to_register(  -2.0), (0b1111_1110, 0));
        assert_eq!(convert_temp_to_register(  -4.0), (0b1111_1100, 0));
        assert_eq!(convert_temp_to_register(  -8.0), (0b1111_1000, 0));
        assert_eq!(convert_temp_to_register( -16.0), (0b1111_0000, 0));
        assert_eq!(convert_temp_to_register( -32.0), (0b1110_0000, 0));
        assert_eq!(convert_temp_to_register( -64.0), (0b1100_0000, 0));
        assert_eq!(convert_temp_to_register(-127.5), (0b1000_0000, 1));
        assert_eq!(convert_temp_to_register(-128.0), (0b1000_0000, 0));
    }
}