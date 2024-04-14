use std::ops::RangeBounds;

use ux::u1;

use crate::{
    bit_read::BitRead,
    error::{Error, Result},
};

/// A slice of bits.  |start_bit_index| is inclusive, |end_bit_index| is exclusive
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitSlice<'a> {
    buf: &'a [u8],
    start_bit_index: u64,
    end_bit_index: u64,
}

#[allow(clippy::len_without_is_empty)]
impl BitSlice<'_> {
    pub(crate) fn new(buf: &[u8], start_bit_index: u64, end_bit_index: u64) -> BitSlice {
        BitSlice {
            buf,
            start_bit_index,
            end_bit_index,
        }
    }

    pub fn len(&self) -> usize {
        (self.end_bit_index - self.start_bit_index) as usize
    }

    /// Retrive the [`u1`] at the given index.  Panics if index is out-of-bounds.
    ///
    /// * `index`: The index.
    pub fn at(&self, index: u64) -> u1 {
        assert!(index < self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        let byte = self.buf[byte_pos as usize];
        get_bit(byte, bit_pos % 8)
    }
}

impl<'a> BitSlice<'a> {
    /// Get a slice of this slice corresponding to the given range.
    ///
    /// Note that 'a is _not_ the lifetime of `self` here, which allows us to avoid scenarios where
    /// a call like `foo.as_bit_slice().get_slice(...)` would complain about returning a reference
    /// to a temporary.
    ///
    /// * `range`: The range.
    pub fn get_slice<T: RangeBounds<u64>>(&self, range: T) -> Result<BitSlice<'a>> {
        let (start_bit_index, end_bit_index) =
            get_start_end_bit_index_from_range(&range, self.len());
        let bit_len = end_bit_index - start_bit_index;
        // Adjust start and end bit indices to be relative to self.start_bit_index
        let start_bit_index = start_bit_index + self.start_bit_index;
        let end_bit_index = start_bit_index + bit_len;

        let start_byte = start_bit_index / 8;
        let end_byte = (end_bit_index - 1) / 8;
        // We now need to adjust the start_bit_index to be relative to the start_byte
        let start_bit_index = start_bit_index - start_byte * 8;
        if end_byte >= self.buf.len() as u64 {
            return Err(Error::SliceOutOfRange {
                len: self.buf.len(),
                slice_start: start_byte,
                slice_end: end_byte,
            });
        }
        Ok(BitSlice::new(
            &self.buf[(start_byte as usize)..=(end_byte as usize)],
            start_bit_index,
            start_bit_index + bit_len,
        ))
    }
}

pub trait AsBitSlice {
    fn as_bit_slice(&self) -> BitSlice;
}

impl AsBitSlice for BitSlice<'_> {
    fn as_bit_slice(&self) -> BitSlice {
        self.clone()
    }
}

impl AsBitSlice for Vec<u8> {
    fn as_bit_slice(&self) -> BitSlice {
        BitSlice {
            buf: &self[..],
            start_bit_index: 0,
            end_bit_index: (self.len() * 8) as u64,
        }
    }
}

impl<T> BitRead for T
where
    T: AsBitSlice,
{
    fn read(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        let slice = self.as_bit_slice();
        let n = slice.len().min(buf.len());
        // TODO: optimize...
        for (i, bit) in buf.iter_mut().enumerate().take(n) {
            *bit = slice.at(i as u64);
        }
        Ok(n)
    }
}

/// Get the start and end bit indices from the given |range|, where |len| represents the length of
/// the item being indexed.  The returned start_bit_index is inclusive and end_bit_index is
/// exclusive.
pub(crate) fn get_start_end_bit_index_from_range<T: RangeBounds<u64>>(
    range: &T,
    len: usize,
) -> (u64, u64) {
    let start_bit_index = match range.start_bound() {
        std::ops::Bound::Included(&s) => s,
        std::ops::Bound::Excluded(s) => s + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let end_bit_index = match range.end_bound() {
        std::ops::Bound::Included(s) => s + 1,
        std::ops::Bound::Excluded(&s) => s,
        // The end bit index is exclusive, so to handle the case where the length is 0 we make sure
        // it's always at least '1'.
        std::ops::Bound::Unbounded => std::cmp::max(len, 1) as u64,
    };
    (start_bit_index, end_bit_index)
}

/// Get the |bit_index| bits of |byte| as a u1
pub(crate) fn get_bit(byte: u8, bit_index: u64) -> u1 {
    let mask = match bit_index {
        0 => 0b10000000,
        1 => 0b01000000,
        2 => 0b00100000,
        3 => 0b00010000,
        4 => 0b00001000,
        5 => 0b00000100,
        6 => 0b00000010,
        7 => 0b00000001,
        _ => unreachable!(),
    };
    let result = byte & mask;
    u1::new(result >> (7 - bit_index))
}
