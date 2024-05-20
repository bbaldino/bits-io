use std::ops::RangeBounds;

use ux::u1;

use crate::{
    bit_read::BitRead,
    error::{Error, Result},
    util::{get_bit, get_start_end_bit_index_from_range},
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

    /// Retrieve the [`u1`] at the given index.  Panics if index is out-of-bounds.
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

impl std::io::Read for BitSlice<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.start_bit_index % 8 {
            0 => {
                let current_byte_pos = (self.start_bit_index / 8) as usize;
                let mut this = &self.buf[current_byte_pos..];
                std::io::Read::read(&mut this, buf)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level read when not on byte boundary",
            )),
        }
    }
}

pub trait AsBitSlice {
    fn as_bit_slice(&self) -> BitSlice;
}

impl<'a> AsBitSlice for BitSlice<'a> {
    fn as_bit_slice(&self) -> BitSlice<'a> {
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
    T: AsBitSlice + std::io::Read,
{
    fn read_bits(&mut self, buf: &mut [ux::u1]) -> std::io::Result<usize> {
        let slice = self.as_bit_slice();
        let n = slice.len().min(buf.len());
        // TODO: optimize...
        for (i, bit) in buf.iter_mut().enumerate().take(n) {
            *bit = slice.at(i as u64);
        }
        Ok(n)
    }
}
