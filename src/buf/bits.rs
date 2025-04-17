use std::ops::{Deref, Range};

use crate::prelude::*;
use bytes::{Bytes, BytesMut};

use super::util::bytes_needed;

/// A cheaply cloneable chunk of contiugous memory, built on top of `[bytes::Bytes`] but providing
/// bit-level operations.  
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bits {
    pub(crate) inner: Bytes,
    /// The start of this instance's view of the underlying storage
    pub(crate) bit_start: usize,
    /// How many bits, from bit_start, are part of this view
    pub(crate) bit_len: usize,
}

impl Bits {
    /// Creates a new empty [`Bits`]
    pub fn from_bytes(bytes: Bytes) -> Self {
        let bit_len = bytes.len() * 8;
        Self {
            inner: bytes,
            bit_start: 0,
            bit_len,
        }
    }

    /// Creates a new [`Bits`] instance from the given slice by copying it.
    pub fn copy_from_slice(bits: &BitSlice) -> Self {
        let bytes_needed = bytes_needed(bits.len());
        let mut bytes = BytesMut::with_capacity(bytes_needed);
        bytes.resize(bytes_needed, 0);

        let target = BitSlice::from_slice_mut(&mut bytes);
        target[..bits.len()].clone_from_bitslice(bits);

        Self {
            inner: bytes.freeze(),
            bit_start: 0,
            bit_len: bits.len(),
        }
    }

    /// Create a slice corresponding to the given range, which is given in bits.  The given range
    /// is relative to the start of the buffer, not the current position.
    pub fn slice(&self, range: Range<usize>) -> Self {
        assert!(
            range.end <= self.bit_start + self.bit_len,
            "Range beyond Bits length"
        );
        Self {
            inner: self.inner.clone(),
            bit_start: self.bit_start + range.start,
            bit_len: range.end - range.start,
        }
    }

    /// Create a slice corresponding to the given range, which is given in bytes.  The given range
    /// is relative to the start of the buffer, not the current position.  Note that the 'start' of
    /// this view may not correspond to a byte boundary on the underlying storage.
    pub fn slice_bytes(&self, range: Range<usize>) -> Self {
        assert!(
            range.end * 8 <= self.bit_start + self.bit_len,
            "Range beyond Bits length"
        );
        let bit_range_start = range.start * 8;
        let bit_range_end = range.end * 8;
        self.slice(bit_range_start..bit_range_end)
    }

    /// Splits the bits into two at the given bit index.
    ///
    /// Afterwards self contains elements [at, len), and the returned Bits contains elements [0,
    /// at).
    pub fn split_to(&mut self, at: usize) -> Self {
        assert!(
            at <= self.bit_len,
            "split_to out of bounds: {:?} must be <= {:?}",
            at,
            self.len()
        );
        let mut ret = self.clone();
        self.inc_start(at);
        ret.bit_len = at;
        ret
    }

    /// Splits the bits into two at the given byte index.  Note that this byte index is relative to
    /// the start of this view, and may not fall on a byte boundary in the underlying storage.
    ///
    /// Afterwards self contains elements [at, len), and the returned Bits contains elements [0,
    /// at).
    pub fn split_to_bytes(&mut self, at: usize) -> Self {
        self.split_to(at * 8)
    }

    /// Splits the bits into two at the given index.
    ///
    /// Afterwards self contains elements [0, at), and the returned Bits contains elements [at,
    /// len).
    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(
            at <= self.bit_len,
            "split_off out of bounds: {:?} must be <= {:?}",
            at,
            self.len()
        );
        let mut ret = self.clone();
        self.bit_len = at;
        ret.inc_start(at);
        ret
    }

    /// Splits the bits into two at the given byte index.  Note that this byte index is relative to
    /// the start of this view, and may not fall on a byte boundary in the underlying storage.
    ///
    /// Afterwards self contains elements [0, at), and the returned Bits contains elements [at,
    /// len).
    pub fn split_off_bytes(&mut self, at: usize) -> Self {
        self.split_off(at * 8)
    }

    /// Shortens the buffer, keeping the first len bits and dropping the rest.
    ///
    /// If len is greater than the buffer’s current length, this has no effect.
    ///
    /// The split_off method can emulate truncate, but this causes the excess bits to be returned instead of dropped.
    pub fn truncate(&mut self, len: usize) {
        if len < self.bit_len {
            self.bit_len = len;
        }
    }

    /// Shortens the buffer, keeping the first len bytes and dropping the rest.
    ///
    /// If len is greater than the buffer’s current length, this has no effect.
    ///
    /// The split_off method can emulate truncate, but this causes the excess bits to be returned instead of dropped.
    pub fn truncate_bytes(&mut self, len: usize) {
        if len * 8 < self.bit_len {
            self.bit_len = len * 8;
        }
    }

    /// Clears the buffer, removing all data.
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Returns the number of bits contained in this `Bits`
    pub fn len(&self) -> usize {
        self.bit_len
    }

    /// Returns the number of _complete_ bytes contained in this `Bits`.  Note that this `Bits` may
    /// contain a number of bits that does not evenly divide into bytes: this method returns the
    /// number of _complete_ bytes, i.e. it does a truncating divide on the number of bits.
    pub fn len_bytes(&self) -> usize {
        self.bit_len / 8
    }

    /// Returns true if the `Bits` has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.bit_len == 0
    }

    /// Move the start point of this view forward by `by` bits.
    pub(crate) fn inc_start(&mut self, by: usize) {
        self.bit_len -= by;
        self.bit_start += by;
    }
}

impl Deref for Bits {
    type Target = BitSlice;

    fn deref(&self) -> &Self::Target {
        BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len].as_ref()
    }
}

impl From<BitVec> for Bits {
    fn from(bv: BitVec) -> Self {
        // As far as I can tell, the bitvec crate does not give any way to get access to the
        // underlying bytes _and_ give the offset at which the bitslice/bitvec starts (since it may
        // not be at the beginning of that underlying storage).  This means we first need to
        // 'left-align' the data that we get here, and the only way to do that is to copy the bits
        // into a new bitvec.
        let bit_len = bv.len();
        let aligned: BitVec = bv.iter().by_vals().collect();
        let bytes = aligned.into_vec();

        Self {
            inner: Bytes::from(bytes),
            bit_start: 0,
            bit_len,
        }
    }
}

impl From<&BitSlice> for Bits {
    fn from(value: &BitSlice) -> Self {
        Bits::from(value.to_bitvec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_from_slice() {
        // Mutable so we can alter it below to make sure a copy was made
        let src = bits![mut 1, 0, 1, 1, 0, 1, 0, 0, 1, 1];

        let bits = Bits::copy_from_slice(src);

        assert_eq!(bits.len(), src.len());
        assert_eq!(src, bits[..]);

        // Ensure the data was copied (not shared)
        src.set(0, false);

        // Original `bits` should not be affected
        assert!(bits[0]);
    }

    #[test]
    fn test_from_bitslice() {
        let slice = bits![1, 1, 0, 1, 1, 0];
        let bits = Bits::from(slice);
        assert_eq!(bits.len(), 6);
        assert_eq!(bits[..], bits![1, 1, 0, 1, 1, 0]);

        // Now make sure a slice that came from a non-byte boundary still works
        let unaligned_slice = &slice[2..];
        println!("{unaligned_slice:?}");
        println!("{:?}", unaligned_slice.to_bitvec());
        let bits = Bits::from(unaligned_slice);
        assert_eq!(bits.len(), 4);
        assert_eq!(bits[..], bits![0, 1, 1, 0]);
    }

    #[test]
    fn test_slice() {
        let raw = Bytes::from_static(&[0b1010_1010, 0b1111_0000]);
        let bits = Bits::from_bytes(raw);

        let head = bits.slice(0..4);
        assert_eq!(head.len(), 4);
        assert_eq!(head[..], bits!(1, 0, 1, 0));

        let mid = bits.slice(4..12);
        assert_eq!(mid.len(), 8);
        assert_eq!(mid[..], bits!(1, 0, 1, 0, 1, 1, 1, 1));

        let tail = bits.slice(12..16);
        assert_eq!(tail.len(), 4);
        assert_eq!(tail[..], bits!(0, 0, 0, 0));

        // A slice which overlaps two existing slices
        let overlapping = bits.slice(10..14);
        assert_eq!(overlapping.len(), 4);
        assert_eq!(overlapping[..], bits!(1, 1, 0, 0));

        // A slice taken from an existing slice who's starting point isn't at a byte boundary
        let slice_of_slice = overlapping.slice(0..2);
        assert_eq!(slice_of_slice.len(), 2);
        assert_eq!(slice_of_slice[..], bits!(1, 1));
    }

    #[test]
    fn test_slice_bytes() {
        #[rustfmt::skip]
        let raw = Bytes::from_static(&[
            0b1010_1010,
            0b1100_1100,
            0b1110_0011,
            0b1111_0000,
        ]);
        let bits = Bits::from_bytes(raw);

        let head = bits.slice_bytes(0..2);
        assert_eq!(head.len(), 16);
        assert_eq!(
            head[..],
            bits!(1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0)
        );

        let mid = bits.slice_bytes(1..3);
        assert_eq!(head.len(), 16);
        assert_eq!(
            mid[..],
            bits!(1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1)
        );

        // Grab a bitslice that starts at a non-byte boundary and make sure a byte slice from that
        // works correctly
        let bitslice = bits.slice(4..32);
        let byte_slice_from_bitslice = bitslice.slice_bytes(0..1);
        assert_eq!(byte_slice_from_bitslice.len(), 8);
        assert_eq!(byte_slice_from_bitslice[..], bits!(1, 0, 1, 0, 1, 1, 0, 0));
    }

    #[test]
    fn test_split_to() {
        let raw = Bytes::from_static(&[0b1010_1010, 0b1111_0000]);
        let mut bits = Bits::from_bytes(raw);

        let head = bits.split_to(7);
        assert_eq!(head.len(), 7);
        assert_eq!(bits.len(), 9);
        assert_eq!(head[..], bits!(1, 0, 1, 0, 1, 0, 1));
        assert_eq!(bits[..], bits!(0, 1, 1, 1, 1, 0, 0, 0, 0));

        // Split again from what was left over
        let head = bits.split_to(3);
        assert_eq!(head.len(), 3);
        assert_eq!(bits.len(), 6);
        assert_eq!(head[..], bits!(0, 1, 1));
        assert_eq!(bits[..], bits!(1, 1, 0, 0, 0, 0));
    }

    #[test]
    fn test_split_to_bytes() {
        // TODO: Add Bits::from_static/from_static_bytes creation method
        #[rustfmt::skip]
        let raw = Bytes::from_static(&[
            0b1010_1010,
            0b1100_1100,
            0b1110_0011,
            0b1111_0000,
        ]);
        let mut bits = Bits::from_bytes(raw);

        let head = bits.split_to_bytes(1);
        assert_eq!(head.len(), 8);
        assert_eq!(bits.len(), 24);
        assert_eq!(head[..], bits!(1, 0, 1, 0, 1, 0, 1, 0));
        assert_eq!(
            bits[..],
            bits!(1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0)
        );

        // Now split on a non-byte boundary
        let _head = bits.split_to(4);
        // Then split_bytes on that and make sure it works
        let head = bits.split_to_bytes(1);
        assert_eq!(head.len(), 8);
        assert_eq!(bits.len(), 12);
        assert_eq!(head[..], bits!(1, 1, 0, 0, 1, 1, 1, 0));
        assert_eq!(bits[..], bits!(0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0));
    }

    #[test]
    fn test_split_off() {
        let raw = Bytes::from_static(&[0b1010_1010, 0b1111_0000]);
        let mut bits = Bits::from_bytes(raw);

        let tail = bits.split_off(7);
        assert_eq!(bits.len(), 7);
        assert_eq!(tail.len(), 9);
        assert_eq!(bits[..], bits!(1, 0, 1, 0, 1, 0, 1));
        assert_eq!(tail[..], bits!(0, 1, 1, 1, 1, 0, 0, 0, 0));

        // Split again from what was left over
        let tail = bits.split_off(3);
        assert_eq!(bits.len(), 3);
        assert_eq!(tail.len(), 4);
        assert_eq!(bits[..], bits!(1, 0, 1));
        assert_eq!(tail[..], bits!(0, 1, 0, 1));
    }

    #[test]
    fn test_split_off_bytes() {
        #[rustfmt::skip]
        let raw = Bytes::from_static(&[
            0b1010_1010,
            0b1100_1100,
            0b1110_0011,
            0b1111_0000,
        ]);
        let mut bits = Bits::from_bytes(raw);

        let tail = bits.split_off_bytes(3);
        // 'tail' is now bits [24, 32), 'bits' is [0, 24)
        assert_eq!(bits.len(), 24);
        assert_eq!(tail.len(), 8);
        assert_eq!(
            bits[..],
            bits!(1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1)
        );
        assert_eq!(tail[..], bits!(1, 1, 1, 1, 0, 0, 0, 0));

        // Now split on a non-byte boundary
        let mut tail = bits.split_off(6);
        // 'tail' is now bits [6, 24), 'bits' is now [0, 6)
        assert_eq!(bits.len(), 6);
        assert_eq!(bits[..], bits!(1, 0, 1, 0, 1, 0));

        // Now split_off_bytes on 'tail' to make sure it works
        let tail_tail = tail.split_off_bytes(1);
        // 'tail_tail' is now bits [14, 24), 'tail' is [6, 14)
        assert_eq!(tail_tail.len(), 10);
        assert_eq!(tail.len(), 8);
        assert_eq!(tail_tail[..], bits!(0, 0, 1, 1, 1, 0, 0, 0, 1, 1));
        assert_eq!(tail[..], bits!(1, 0, 1, 1, 0, 0, 1, 1));
    }

    #[test]
    fn test_truncate() {
        #[rustfmt::skip]
        let raw = Bytes::from_static(&[
            0b1010_1010,
            0b1100_1100,
            0b1110_0011,
            0b1111_0000,
        ]);
        let mut bits = Bits::from_bytes(raw);

        bits.truncate(10);
        assert_eq!(bits.len(), 10);
        assert_eq!(bits[..], bits![1, 0, 1, 0, 1, 0, 1, 0, 1, 1]);
    }

    #[test]
    fn test_truncate_bytes() {
        #[rustfmt::skip]
        let raw = Bytes::from_static(&[
            0b1010_1010,
            0b1100_1100,
            0b1110_0011,
            0b1111_0000,
        ]);
        let mut bits = Bits::from_bytes(raw);

        bits.truncate_bytes(2);
        assert_eq!(bits.len(), 16);
        assert_eq!(
            bits[..],
            bits![1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0]
        );
    }
}
