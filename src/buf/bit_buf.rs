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

    /// Returns a slice starting at the current position and of length between 0 and
    /// `BitBuf::remaining`.  Note that this _can_ return a shorter slice.
    fn chunk(&self) -> &BitSlice;
}
