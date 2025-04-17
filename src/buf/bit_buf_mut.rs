use crate::prelude::*;

pub trait BitBufMut {
    /// Advance the internal cursor of the BitBufMut
    ///
    /// The next call to chunk_mut will return a slice starting cnt bytes further into the
    /// underlying buffer.
    fn advance_mut(&mut self, count: usize);

    /// Returns a mutable slice starting at the current BitBufMut position and of length between 0
    /// and BitBufMut::remaining_mut(). Note that this can be shorter than the whole remainder of
    /// the buffer (this allows non-continuous implementation).
    ///
    /// This is a lower level function. Most operations are done with other functions.
    ///
    /// The returned byte slice may represent uninitialized memory and should not be read from.
    fn chunk_mut(&mut self) -> &mut BitSlice;

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
    fn remaining_mut(&self) -> usize {
        self.remaining_mut_bytes().saturating_mul(8)
    }

    /// Returns the number of bytes that can be written from the current position until the end of
    /// the buffer is reached.  
    ///
    /// This value is greater than or equal to the length of the slice returned by chunk_mut().
    ///
    /// Writing to a BitBufMut may involve allocating more memory on the fly. Implementations may
    /// fail before reaching the number of bytes indicated by this method if they encounter an
    /// allocation failure.
    fn remaining_mut_bytes(&self) -> usize;

    fn put_slice(&mut self, mut src: &BitSlice) {
        if self.remaining_mut() < src.len() {
            panic!(
                "Not enough room to put slice of size (needed {}, have {}",
                src.len(),
                self.remaining_mut()
            );
        }
        while !src.is_empty() {
            let dest = self.chunk_mut();
            let count = usize::min(src.len(), dest.len());

            dest[..count].copy_from_bitslice(&src[..count]);
            src = &src[..count];

            self.advance_mut(count);
        }
    }
}
