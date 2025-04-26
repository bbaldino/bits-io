use crate::prelude::*;

use super::take::Take;

pub trait BitBuf {
    /// Advance the internal cursor of the `BitBuf` by `count` bits.
    ///
    /// The next call to chunk() will return a slice starting count bits further into the
    /// underlying buffer.
    fn advance_bits(&mut self, count: usize);

    /// Advance the internal cursor of the `BitBuf` by `count` bytes.
    ///
    /// The next call to chunk() will return a slice starting count bytes further into the
    /// underlying buffer.
    fn advance_bytes(&mut self, count: usize) {
        self.advance_bits(count * 8)
    }

    /// Returns the number of bits between the current position and the end of the buffer.
    ///
    /// This value is greater than or equal to the length of the slice returned by `chunk`.
    fn remaining_bits(&self) -> usize;

    ///  Return the number of _full_ bytes between the current position and the end of the buffer.
    fn remaining_bytes(&self) -> usize {
        self.remaining_bits() / 8
    }

    /// Returns a [`BitSlice`] starting at the current position and of length between 0 and
    /// `BitBuf::remaining`.  Note that this _can_ return a shorter slice.
    fn chunk_bits(&self) -> &BitSlice;

    /// Returns a slice of bytes starting at the current position and of length between 0 and
    /// `BitBuf::remaining_bytes`.  Note that this _can_ return a shorter slice.
    fn chunk_bytes(&self) -> &[u8];

    /// Copy bits from `self` into `dest`.
    ///
    /// The cursor is advanced by the number of bits copied.  `self` must have enough remaining
    /// bits to fill `dest`.
    fn copy_to_bit_slice(&mut self, dest: &mut BitSlice) {
        self.try_copy_to_bit_slice(dest).unwrap()
    }

    fn try_copy_to_bit_slice(&mut self, mut dest: &mut BitSlice) -> std::io::Result<()> {
        if self.remaining_bits() < dest.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining bytes ({}) are less than the size of the dest ({})",
                    self.remaining_bytes(),
                    dest.len()
                ),
            ));
        }

        while !dest.is_empty() {
            let src = self.chunk_bits();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_bitslice(&src[..count]);

            dest = &mut dest[count..];

            self.advance_bits(count);
        }

        Ok(())
    }

    /// Copy bytes from `self` into `dest`.  Call should call `byte_aligned()` beforehand to ensure
    /// buffer is fully byte-aligned before calling, call may panic if buffer isn't byte-aligned.
    ///
    /// The cursor is advanced by the number of bytes copied.  `self` must have enough remaining
    /// bytes to fill `dest`.
    fn copy_to_slice_bytes(&mut self, dest: &mut [u8]) {
        self.try_copy_to_slice_bytes(dest).unwrap()
    }

    /// Try to copy bytes from `self` into `dest`.  Returns error if `self` is not big enough to
    /// fill `dest` or if self is not fully byte-aligned (start and end points both falling on byte
    /// boundaries).
    fn try_copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) -> std::io::Result<()> {
        if !self.byte_aligned() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buf beginning and end must both be byte-aligned",
            ));
        }
        if self.remaining_bytes() < dest.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                format!(
                    "Remaining bytes ({}) are less than the size of the dest ({})",
                    self.remaining_bytes(),
                    dest.len()
                ),
            ));
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance_bytes(count);
        }

        Ok(())
    }

    fn take_bits(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, limit)
    }

    fn take_bytes(self, limit: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, limit * 8)
    }

    /// Returns whether or not this `BitBuf` is fully byte-aligned (beginning and end) with the
    /// underlying storage.
    fn byte_aligned(&self) -> bool;
}
