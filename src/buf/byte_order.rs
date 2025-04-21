use bitvec::field::BitField;
use funty::Integral;

use crate::prelude::*;

/// This trait defines operations to load and store integral values from a buffer, and enables
/// implementing them in different ways for the different byte orders (Big Endian and Little
/// Endian).
pub trait ByteOrder {
    fn load<O: BitStore, U: Integral>(src: &BitSlice<O>) -> U;
    fn store<O: BitStore, U: Integral>(dest: &mut BitSlice<O>, value: U);
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

impl ByteOrder for BigEndian {
    fn load<O: BitStore, U: Integral>(src: &BitSlice<O>) -> U {
        src.load_be()
    }

    fn store<O: BitStore, U: Integral>(dest: &mut BitSlice<O>, value: U) {
        dest.store_be(value);
    }
}

impl ByteOrder for LittleEndian {
    fn load<O: BitStore, U: Integral>(src: &BitSlice<O>) -> U {
        src.load_le()
    }

    fn store<O: BitStore, U: Integral>(dest: &mut BitSlice<O>, value: U) {
        dest.store_le(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: more test cases here (different data sizes)

    #[test]
    fn test_big_endian_write() {
        let mut buf = [0u8; 2];
        {
            // Simulate writing 4 bits
            let dest = &mut BitSlice::from_slice_mut(&mut buf);
            BigEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_be_bytes(buf);
            assert_eq!(value, 0xABCD);
        }

        // Now test that writing into a BitSlice<BitSafeU8> also works.
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            let (_, dest) = unsafe { dest.split_at_unchecked_mut(0) };
            BigEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_be_bytes(buf);
            assert_eq!(value, 0xABCD);
        }
    }

    #[test]
    fn test_big_endian_read() {
        let value = 0xABCDu16;
        let value_bytes = value.to_be_bytes();
        let src = BitSlice::from_slice(&value_bytes);
        let read_value: u16 = BigEndian::load(src);
        assert_eq!(value, read_value);
    }

    #[test]
    fn test_little_endian_write() {
        let mut buf = [0u8; 2];
        {
            // Simulate writing 4 bits
            let dest = &mut BitSlice::from_slice_mut(&mut buf);
            LittleEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_le_bytes(buf);
            assert_eq!(value, 0xABCD);
        }

        // Now test that writing into a BitSlice<BitSafeU8> also works.
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            let (_, dest) = unsafe { dest.split_at_unchecked_mut(0) };
            LittleEndian::store(&mut dest[..16], 0xABCDu16);
            let value = u16::from_le_bytes(buf);
            assert_eq!(value, 0xABCD);
        }
    }

    #[test]
    fn test_little_endian_read() {
        let value = 0xABCDu16;
        let value_bytes = value.to_le_bytes();
        let src = BitSlice::from_slice(&value_bytes);
        let read_value: u16 = LittleEndian::load(src);
        assert_eq!(value, read_value);
    }
}
