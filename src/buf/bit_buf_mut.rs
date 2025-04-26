use bytes::buf::UninitSlice;

use crate::prelude::*;

pub trait BitBufMut {
    /// Advance the internal cursor of the BitBufMut by `count` bits.
    ///
    /// The next call to chunk_mut will return a slice starting `count` bits further into the
    /// underlying buffer.
    fn advance_mut_bits(&mut self, count: usize);

    /// Advance the internal cursor of the BitBufMut by `count` bytes.
    ///
    /// The next call to chunk_mut will return a slice starting `count` bytes further into the
    /// underlying buffer.
    fn advance_mut_bytes(&mut self, count: usize) {
        self.advance_mut_bits(count * 8);
    }

    /// Returns a mutable `BitSlice` starting at the current `BitBufMut` position and of length
    /// between 0 and BitBufMut::remaining_mut(). Note that this can be shorter than the whole
    /// remainder of the buffer (this allows non-continuous implementation).
    ///
    /// This is a lower level function. Most operations are done with other functions.
    ///
    /// The returned byte slice may represent uninitialized memory and should not be read from.
    fn chunk_mut_bits(&mut self) -> &mut BitSlice;

    /// Returns a mutable `UninitSlice` starting at the current `BitBufMut` position and of length
    /// between 0 and BitBufMut::remaining_mut(). Note that this can be shorter than the whole
    /// remainder of the buffer (this allows non-continuous implementation).  This `BitBufMut`
    /// must be fully byte-aligned for this to work: caller should check `byte_aligned` before
    /// calling.
    ///
    /// This is a lower level function. Most operations are done with other functions.
    ///
    /// The returned byte slice may represent uninitialized memory and should not be read from.
    fn chunk_mut_bytes(&mut self) -> &mut UninitSlice;

    /// Returns the number of bits that can be written from the current position until the end of
    /// the buffer is reached.  Note that the returned value may under-represent the remainin
    /// amount: we are returning the value in bits but if the underlying storage is in bytes then
    /// the result here will be under-represented by a factor of 8.  `remaining_mut_bytes` will
    /// give a more accurate view of how much space (in bytes) is remaining.
    ///
    /// This value is greater than or equal to the length of the slice returned by chunk_mut().
    ///
    /// Writing to a BitBufMut may involve allocating more memory on the fly. Implementations may
    /// fail before reaching the number of bytes indicated by this method if they encounter an
    /// allocation failure.
    fn remaining_mut_bits(&self) -> usize;

    /// Returns the number of _full_ bytes that can be written from the current position until the
    /// end of the buffer is reached.  
    ///
    /// This value is greater than or equal to the length of the slice returned by chunk_mut().
    ///
    /// Writing to a BitBufMut may involve allocating more memory on the fly. Implementations may
    /// fail before reaching the number of bytes indicated by this method if they encounter an
    /// allocation failure.
    fn remaining_mut_bytes(&self) -> usize {
        self.remaining_mut_bits() / 8
    }

    /// Transfer bits into `self` from `src` and advance the cursor by the number of bits written.
    ///
    /// `self` must have enough remaining capacity to contain all of `src`.
    fn put_bit_slice(&mut self, src: &BitSlice) {
        self.try_put_bit_slice(src).unwrap()
    }

    /// Try to transfer bits info `self` from `src` and advance the cursor by the number of bits
    /// written.
    ///
    /// Returns an error if `self` doesn't have enough remaining capacity to contain all of `src`.
    fn try_put_bit_slice(&mut self, mut src: &BitSlice) -> std::io::Result<()> {
        if self.remaining_mut_bits() < src.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining  bits ({}) are less than the size of the source ({})",
                    self.remaining_mut_bits(),
                    src.len()
                ),
            ));
        }
        while !src.is_empty() {
            let dest = self.chunk_mut_bits();
            let count = usize::min(src.len(), dest.len());

            dest[..count].copy_from_bitslice(&src[..count]);
            src = &src[count..];

            self.advance_mut_bits(count);
        }

        Ok(())
    }

    fn try_put_slice_bytes(&mut self, mut src: &[u8]) -> std::io::Result<()> {
        if !self.byte_aligned_mut() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "BitBuf beginning and end must both be byte-aligned",
            ));
        }
        if self.remaining_mut_bytes() < src.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining  bytes ({}) are less than the size of the source ({})",
                    self.remaining_mut_bytes(),
                    src.len()
                ),
            ));
        }
        while !src.is_empty() {
            let dest = self.chunk_mut_bytes();
            let count = usize::min(src.len(), dest.len());

            dest[..count].copy_from_slice(&src[..count]);
            src = &src[count..];

            self.advance_mut_bytes(count);
        }

        Ok(())
    }

    /// Returns whether or not this `BitBufMut` is fully byte-aligned (beginning and end) with the
    /// underlying storage.
    fn byte_aligned_mut(&self) -> bool;
}
