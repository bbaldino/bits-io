use crate::prelude::*;

pub trait BitBuf {
    /// Advance the internal cursor of the `BitBuf` by `count` bits.
    ///
    /// The next call to chunk() will return a slice starting count bits further into the
    /// underlying buffer.
    fn advance(&mut self, count: usize);

    /// Returns the number of bits between the current position and the end of the buffer.
    ///
    /// This value is greater than or equal to the length of the slice returned by `chunk`.
    fn remaining(&self) -> usize;

    ///  Return the number of _full_ bytes between the current position and the end of the buffer.
    fn remaining_bytes(&self) -> usize {
        self.remaining() / 8
    }

    /// Returns a [`BitSlice`] starting at the current position and of length between 0 and
    /// `BitBuf::remaining`.  Note that this _can_ return a shorter slice.
    fn chunk(&self) -> &BitSlice;

    /// Returns a slice of bytes starting at the current position and of length between 0 and
    /// `BitBuf::remaining_bytes`.  Note that this _can_ return a shorter slice.
    fn chunk_bytes(&self) -> &[u8];

    /// Copy bits from `self` into `dest`.
    ///
    /// The cursor is advanced by the number of bits copied.  `self` must have enough remaining
    /// bits to fill `dest`.
    fn copy_to_slice(&mut self, mut dest: &mut BitSlice) {
        if self.remaining() < dest.len() {
            panic!(
                "Remaining bits ({}) are less than the size of dest ({})",
                self.remaining(),
                dest.len()
            );
        }

        while !dest.is_empty() {
            let src = self.chunk();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_bitslice(&src[..count]);

            dest = &mut dest[count..];

            self.advance(count);
        }
    }

    fn copy_to_slice_bytes(&mut self, dest: &mut [u8]);
}
