use std::ops::RangeBounds;

use ux::u1;

use crate::{
    bit_slice::{AsBitSlice, BitSlice},
    bit_write::BitWrite,
    error::{Error, Result},
    util::{get_bit, get_start_end_bit_index_from_range, set_bit},
};

/// A mutable slice of bits.  |start_bit_index| is inclusive, |end_bit_index| is exclusive
#[derive(Debug, Eq, PartialEq)]
pub struct BitSliceMut<'a> {
    buf: &'a mut [u8],
    start_bit_index: u64,
    end_bit_index: u64,
}

impl BitSliceMut<'_> {
    pub(crate) fn new(buf: &mut [u8], start_bit_index: u64, end_bit_index: u64) -> BitSliceMut {
        BitSliceMut {
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

    pub fn set(&mut self, index: u64, value: u1) {
        assert!(index < self.end_bit_index);
        let bit_pos = self.start_bit_index + index;
        let byte_pos = bit_pos / 8;
        // Now make bit_pos relative to the byte
        let bit_pos = bit_pos % 8;
        let byte = &mut self.buf[byte_pos as usize];
        set_bit(byte, bit_pos, value);
    }
}

impl<'a> BitSliceMut<'a> {
    /// Get a slice of this slice corresponding to the given range.
    ///
    /// Note that 'a is _not_ the lifetime of `self` here, which allows us to avoid scenarios where
    /// a call like `foo.as_bit_slice().get_slice(...)` would complain about returning a reference
    /// to a temporary.
    ///
    /// * `range`: The range.
    pub fn get_slice<T: RangeBounds<u64>>(&self, range: T) -> Result<BitSlice> {
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

    pub fn get_slice_mut<T: RangeBounds<u64>>(&mut self, range: T) -> Result<BitSliceMut> {
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
        Ok(BitSliceMut::new(
            &mut self.buf[(start_byte as usize)..=(end_byte as usize)],
            start_bit_index,
            start_bit_index + bit_len,
        ))
    }
}

impl<'a> AsBitSlice for BitSliceMut<'a> {
    fn as_bit_slice(&self) -> BitSlice {
        BitSlice::new(self.buf, self.start_bit_index, self.end_bit_index)
    }
}

impl std::io::Read for BitSliceMut<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        (self.as_bit_slice()).read(buf)
    }
}

impl std::io::Write for BitSliceMut<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.start_bit_index % 8 {
            0 => {
                let current_byte_pos = (self.start_bit_index / 8) as usize;
                let mut this = &mut self.buf[current_byte_pos..];
                std::io::Write::write(&mut this, buf)
            }
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Attempted byte-level write when not on byte boundary",
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub trait AsBitSliceMut<'a>: AsBitSlice {
    fn as_bit_slice_mut<'b>(&'a mut self) -> BitSliceMut<'b>
    where
        'a: 'b;
}

impl<'a> AsBitSliceMut<'a> for BitSliceMut<'a> {
    fn as_bit_slice_mut<'b>(&'a mut self) -> BitSliceMut<'b>
    where
        'a: 'b,
    {
        BitSliceMut {
            buf: self.buf,
            start_bit_index: self.start_bit_index,
            end_bit_index: self.end_bit_index,
        }
    }
}

impl<'a> AsBitSliceMut<'a> for Vec<u8> {
    fn as_bit_slice_mut<'b>(&'a mut self) -> BitSliceMut<'b>
    where
        'a: 'b,
    {
        let len = (self.len() * 8) as u64;
        BitSliceMut {
            buf: &mut self[..],
            start_bit_index: 0,
            end_bit_index: len,
        }
    }
}

impl<T> BitWrite for T
where
    T: for<'a> AsBitSliceMut<'a> + std::io::Write,
{
    fn write_bits(&mut self, buf: &[u1]) -> std::io::Result<usize> {
        let mut slice = self.as_bit_slice_mut();
        let n = slice.len().min(buf.len());
        for (i, bit) in buf.iter().enumerate().take(n) {
            slice.set(i as u64, *bit)
        }
        Ok(n)
    }
}
