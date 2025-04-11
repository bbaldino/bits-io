use crate::prelude::*;

pub trait ByteOrder {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32);
    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32;
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

impl ByteOrder for BigEndian {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32) {
        let n = bits.len();
        assert!(n <= 32, "cannot write more than 32 bits");

        for i in 0..n {
            // Extract bit starting from MSB
            let bit = (value >> (n - 1 - i)) & 1;
            bits.set(i, bit != 0);
        }
    }

    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32 {
        let n = bits.len();
        assert!(n <= 32, "cannot read more than 32 bits into a u32");

        bits.iter()
            .fold(0u32, |acc, bit| (acc << 1) | (*bit as u32))
    }
}

impl ByteOrder for LittleEndian {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32) {
        let n = bits.len();
        assert!(n <= 32, "cannot write more than 32 bits");

        for i in 0..n {
            // Extract bit starting from LSB
            let bit = (value >> i) & 1;
            bits.set(i, bit != 0);
        }
    }

    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32 {
        let n = bits.len();
        assert!(n <= 32, "cannot read more than 32 bits into a u32");

        bits.iter()
            .rev()
            .fold(0u32, |acc, bit| (acc << 1) | (*bit as u32))
    }
}

#[cfg(test)]
mod tests {
    use crate::{bit_read_exts::BitReadExts, bit_write_exts::BitWriteExts};

    use super::*;
    // use bitvec::prelude::*;
    #[test]
    fn test_big_endian_read_write() {
        let test_cases = [
            (0b1, 1),
            (0b10, 2),
            (0b10101010, 8),
            (0b1010101010101010, 16),
            (0xDEADBEEF, 32),
        ];

        for &(value, bits) in &test_cases {
            let mut cursor = BitCursor::new(vec![0u8; 8]);

            if bits <= 8 {
                cursor.write_u8(value as u8).unwrap();
            } else if bits <= 16 {
                cursor.write_u16::<BigEndian>(value as u16).unwrap();
            } else {
                cursor.write_u32::<BigEndian>(value as u32).unwrap();
            }

            cursor.set_position(0);

            let read_value = if bits <= 8 {
                cursor.read_u8().unwrap() as u64
            } else if bits <= 16 {
                cursor.read_u16::<BigEndian>().unwrap() as u64
            } else {
                cursor.read_u32::<BigEndian>().unwrap() as u64
            };

            assert_eq!(value, read_value, "BigEndian failed for {} bits", bits);
        }
    }

    #[test]
    fn test_little_endian_read_write() {
        let test_cases = [
            (0b1, 1),
            (0b10, 2),
            (0b10101010, 8),
            (0b1010101010101010, 16),
            (0xDEADBEEF, 32),
        ];

        for &(value, bits) in &test_cases {
            let mut cursor = BitCursor::new(vec![0u8; 8]);

            if bits <= 8 {
                cursor.write_u8(value as u8).unwrap();
            } else if bits <= 16 {
                cursor.write_u16::<LittleEndian>(value as u16).unwrap();
            } else {
                cursor.write_u32::<LittleEndian>(value as u32).unwrap();
            }

            cursor.set_position(0);

            let read_value = if bits <= 8 {
                cursor.read_u8().unwrap() as u64
            } else if bits <= 16 {
                cursor.read_u16::<LittleEndian>().unwrap() as u64
            } else {
                cursor.read_u32::<LittleEndian>().unwrap() as u64
            };

            assert_eq!(value, read_value, "LittleEndian failed for {} bits", bits);
        }
    }
}
