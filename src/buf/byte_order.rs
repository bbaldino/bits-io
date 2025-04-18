use bitvec::{access::BitSafeU8, order::Msb0, view::BitView};

use crate::prelude::*;

pub trait ByteOrder {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32);
    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32;
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

/// Copy data from one `BitSlice` to another.  This trait exists to facilitate copying data between
/// `BitSlice`s with _different_ `BitStore` values (specifically: u8 vs BitSafeU8).  `BitSlice`'s
/// built-in `copy_from_slice` method requires that both the source and dest have the exact same
/// `BitStore` type.  Since we sometimes deal with `BitSlice<BitSafeU8`> but the u32 values will
/// always be view as `BitSlice<u8>`, we need a way to copy data from a `BitSlice<u8>` into a
/// `BitSlice<BitSafeU8>`.  This trait allows specialized implementations for the `BitSlice<u8`> ->
/// `BitSlice<u8>` and `BitSlice<u8>` -> `BitSlice<BitSafeU8>` cases.
///
/// Copy data to a `BitSlice` whose `BitStore` type is controlled by the caller from a
/// `&mut BitSlice<u8>`.  The source must be mutable to support potentially 'transforming' it
/// into a `BitSlice` using a different `BitStore`.
pub trait CopyFromBitSlice {
    /// Copy data from `src` into `dst`.  Note that `src` needs to be mutable because in order to
    /// convert it into a `BitSlice<BitSafeU8>`, we need to call some form of split_mut on it.
    fn copy_from_slice(dest: &mut BitSlice<Self>, src: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized;
}

impl CopyFromBitSlice for u8 {
    fn copy_from_slice(dest: &mut BitSlice<Self>, src: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized,
    {
        dest.copy_from_bitslice(src);
    }
}

impl CopyFromBitSlice for BitSafeU8 {
    fn copy_from_slice(dest: &mut BitSlice<Self>, src: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized,
    {
        // We need to write into a `BitSlice<BitSafeU8>`, so we need to convert `src` to
        // `BitSlice<BitSafeU8>`.  There's no public method for creating `BitSafeU8` instances, so
        // our only option is to make a call to `split_at_mut` to get one.
        //
        // SAFETY - we're splitting at 0 which we know will always succeed.
        let (_, data) = unsafe { src.split_at_unchecked_mut(0) };
        dest.copy_from_bitslice(data);
    }
}

/// This trait looks a lot like `CopyFromBitSlice` but there's an important difference: here it's
/// the source's `BitStore` type that differs between the impls as opposed to the dest's.  When
/// we're reading from a buffer, we have 'control' of the opposite buffer as compared to writing
/// (here we control the dest, not the source) so we need a 'mirrored' trait to handle the read
/// cases.
///
/// Copy data from a `BitSlice` whose `BitStore` type is controlled by the caller from a `&mut
/// BitSlice<u8>`
pub trait CopyToBitSlice {
    fn copy_to_slice(src: &BitSlice<Self>, dest: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized;
}

impl CopyToBitSlice for u8 {
    fn copy_to_slice(src: &BitSlice<Self>, dest: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized,
    {
        dest.copy_from_bitslice(src);
    }
}

impl CopyToBitSlice for BitSafeU8 {
    fn copy_to_slice(src: &BitSlice<Self>, dest: &mut BitSlice<u8>)
    where
        Self: BitStore + Sized,
    {
        let (_, dest) = unsafe { dest.split_at_unchecked_mut(0) };
        dest.copy_from_bitslice(src);
    }
}

impl ByteOrder for BigEndian {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32) {
        assert!(bits.len() <= 32, "cannot write more than 32 bits");
        let mut value_be_bytes = value.to_be_bytes();
        let value_slice = &mut value_be_bytes.view_bits_mut::<Msb0>()[(32 - bits.len())..];
        <O as CopyFromBitSlice>::copy_from_slice(bits, value_slice);
    }

    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32 {
        assert!(bits.len() <= 32, "cannot read more than 32 bits into a u32");

        let mut buf = [0u8; 4];
        let dest = &mut BitSlice::from_slice_mut(&mut buf)[..bits.len()];
        <O as CopyToBitSlice>::copy_to_slice(bits, dest);
        // Now the bits are left-aligned in dest (and therefore in buf) and we can use them
        // directly to convert them to a u32.
        u32::from_be_bytes(buf)
    }
}

impl ByteOrder for LittleEndian {
    fn write_u32_to_bits<O: BitStore>(bits: &mut BitSlice<O>, value: u32) {
        assert!(bits.len() <= 32, "cannot write more than 32 bits");
        let mut value_le_bytes = value.to_le_bytes();
        let value_slice = &mut value_le_bytes.view_bits_mut::<Msb0>()[(32 - bits.len())..];
        <O as CopyFromBitSlice>::copy_from_slice(bits, value_slice);
    }

    fn read_u32_from_bits<O: BitStore>(bits: &BitSlice<O>) -> u32 {
        assert!(bits.len() <= 32, "cannot read more than 32 bits into a u32");

        let mut buf = [0u8; 4];
        let dest = &mut BitSlice::from_slice_mut(&mut buf)[..bits.len()];
        <O as CopyToBitSlice>::copy_to_slice(bits, dest);
        // Now the bits are left-aaligned in dest (and therefore in buf) and we can use them
        // directly to convert them to a u32.
        u32::from_le_bytes(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_endian_write() {
        let mut buf = [0u8; 4];
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            BigEndian::write_u32_to_bits(dest, 0xABCDu32);
            let value = u32::from_be_bytes(buf);
            assert_eq!(value, 0xABCD);
        }

        // Now test that writing into a BitSlice<BitSafeU8> also works.
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            let (_, dest) = unsafe { dest.split_at_unchecked_mut(0) };
            BigEndian::write_u32_to_bits(dest, 0xABCDu32);
            let value = u32::from_be_bytes(buf);
            assert_eq!(value, 0xABCD);
        }
    }

    #[test]
    fn test_big_endian_read() {
        let value = 0xABCDu32;
        let value_bytes = value.to_be_bytes();
        let src = BitSlice::from_slice(&value_bytes);
        let read_value = BigEndian::read_u32_from_bits(src);
        assert_eq!(value, read_value);
    }

    #[test]
    fn test_little_endian_write() {
        let mut buf = [0u8; 4];
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            LittleEndian::write_u32_to_bits(dest, 0xABCDu32);
            let value = u32::from_le_bytes(buf);
            assert_eq!(value, 0xABCD);
        }

        // Now test that writing into a BitSlice<BitSafeU8> also works.
        {
            let dest = BitSlice::from_slice_mut(&mut buf);
            let (_, dest) = unsafe { dest.split_at_unchecked_mut(0) };
            LittleEndian::write_u32_to_bits(dest, 0xABCDu32);
            let value = u32::from_le_bytes(buf);
            assert_eq!(value, 0xABCD);
        }
    }

    #[test]
    fn test_little_endian_read() {
        {
            let value = 0xABCDEF01u32;
            let value_bytes = value.to_le_bytes();
            let src = BitSlice::from_slice(&value_bytes);
            let read_value = LittleEndian::read_u32_from_bits(src);
            assert_eq!(value, read_value);
        }
        {
            let value = 0xABCDu32;
            let value_bytes = value.to_le_bytes();
            let src = BitSlice::from_slice(&value_bytes);
            let read_value = LittleEndian::read_u32_from_bits(src);
            assert_eq!(value, read_value);
        }
    }
}
