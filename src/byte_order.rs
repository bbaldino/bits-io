use crate::prelude::*;

pub trait ByteOrder {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSliceMut<O>, value: u32);
    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32;
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

impl ByteOrder for BigEndian {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSliceMut<O>, value: u32) {
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
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSliceMut<O>, value: u32) {
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
    use super::*;
    // use bitvec::prelude::*;

    #[test]
    fn test_big_endian_read_write() {
        let cases = [
            (0b1, 1),
            (0b10, 2),
            (0b10101010, 8),
            (0b1010101010101010, 16),
            (0xDEADBEEF, 32),
        ];

        for &(value, n) in &cases {
            let mut tmp = BitVec::repeat(false, n);
            BigEndian::write_u32_to_bits(tmp.as_mut_bitslice(), value);

            let read_back = BigEndian::read_u32_from_bits(tmp.as_bitslice());

            assert_eq!(read_back, value, "BigEndian failed for {} bits", n);
        }
    }

    #[test]
    fn test_little_endian_read_write() {
        let cases = [
            (0b1, 1),
            (0b10, 2),
            (0b10101010, 8),
            (0b1010101010101010, 16),
            (0xDEADBEEF, 32),
        ];

        for &(value, n) in &cases {
            let mut tmp = BitVec::repeat(false, n);
            LittleEndian::write_u32_to_bits(tmp.as_mut_bitslice(), value);

            let read_back = LittleEndian::read_u32_from_bits(tmp.as_bitslice());

            assert_eq!(read_back, value, "LittleEndian failed for {} bits", n);
        }
    }
}
