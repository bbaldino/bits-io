use crate::prelude::*;

pub trait BitBuf {
    /// Advance the internal cursor of the `BitBuf` by `count` bits.
    ///
    /// The next call to chunk() will return a slice starting count bits further into the
    /// underlying buffer.
    fn advance(&mut self, count: usize);

    /// Advance the internal cursor of the `BitBuf` by `count` bytes.
    ///
    /// The next call to chunk() will return a slice starting count bytes further into the
    /// underlying buffer.
    fn advance_bytes(&mut self, count: usize) {
        self.advance(count * 8)
    }

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
    /// TODO: try_ version of this
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
    fn try_copy_to_slice_bytes(&mut self, dest: &mut [u8]) -> std::io::Result<()>;

    /// Returns whether or not this `BitBuf` is fully byte-aligned (beginning and end) with the
    /// underlying storage.
    fn byte_aligned(&self) -> bool;
}
