use std::ops::{Deref, DerefMut};

use crate::prelude::*;

use bytes::{Bytes, BytesMut};

use super::util::bytes_needed;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitsMut {
    pub(crate) inner: BytesMut,
    /// The start of this instance's view of the underlying storage
    pub(crate) bit_start: usize,
    /// How many bits, from bit_start, are part of this view
    pub(crate) bit_len: usize,
    /// This view's capacity
    pub(crate) capacity: usize,
}

impl BitsMut {
    /// Creates a new `BitsMut` with default capacity.  Resulting object has length 0 and
    /// unspecified capacity.
    pub fn new() -> Self {
        BitsMut::with_capacity(0)
    }

    pub fn from_bytes_mut(bytes_mut: BytesMut) -> Self {
        let capacity = bytes_mut.capacity() * 8;
        let bit_len = bytes_mut.len() * 8;
        Self {
            inner: bytes_mut,
            bit_start: 0,
            bit_len,
            capacity,
        }
    }

    /// Creates a new `BitsMut` with the specified capacity in bits.  The returned `BitsMut` will
    /// be able to hold at least `capacity` bits without reallocating.
    ///
    /// It is important to note that this function does not specify the length of the returned
    /// `BitsMut``, but only the capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let byte_capacity = bytes_needed(capacity);
        Self {
            inner: BytesMut::with_capacity(byte_capacity),
            bit_start: 0,
            bit_len: 0,
            capacity,
        }
    }

    /// Creates a new `BitsMut` with the specified capacity in bytes.  The returned `BitsMut` will
    /// be able to hold at least `capacity` bytes without reallocating.
    ///
    /// It is important to note that this function does not specify the length of the returned
    /// `BitsMut``, but only the capacity.
    pub fn with_capacity_bytes(capacity: usize) -> Self {
        Self::with_capacity(capacity * 8)
    }

    /// Creates a new `BitsMut` containing `len` zeros.
    ///
    /// The resulting object has a length of `len` and a capacity greater than or equal to `len`.
    /// The entire length of the object will be filled with zeros.
    pub fn zeroed(len: usize) -> Self {
        let num_bytes = bytes_needed(len);
        Self {
            inner: BytesMut::zeroed(num_bytes),
            bit_start: 0,
            bit_len: len,
            capacity: len,
        }
    }

    /// Creates a new `BitsMut` containing `len` _bytes_ of zeros.
    ///
    /// The resulting object has a length of `len` * 8 and a capacity greater than or equal to `len`
    /// * 8. The entire length of the object will be filled with zeros.
    pub fn zeroed_bytes(len: usize) -> Self {
        Self::zeroed(len * 8)
    }

    /// Converts self into an immutable [`Bits`].
    /// The conversion is zero cost and is used to indicate that the slice referenced by the handle
    /// will no longer be mutated. Once the conversion is done, the handle can be cloned and shared
    /// across threads.
    pub fn freeze(self) -> Bits {
        Bits {
            inner: self.inner.freeze(),
            bit_start: self.bit_start,
            bit_len: self.bit_len,
        }
    }

    /// Appends given bytes to this BytesMut.
    ///
    /// If this `BitsMut` object does not have enough capacity, it is resized first.
    pub fn extend_from_slice(&mut self, slice: &BitSlice) {
        let count = slice.len();
        self.reserve(count);

        let dest = self.spare_capacity_mut();
        assert!(dest.len() >= count);
        dest[..count].copy_from_bitslice(slice);

        self.advance_mut(count);
    }

    /// Returns the remaining spare capacity of the buffer as a `&mut BitSlice`.
    ///
    /// The returned slice can be used to fill the buffer with data (e.g. by reading from a file)
    /// before marking the data as initialized using the set_len method.
    ///
    /// Note that the returned slice is *uninitialized*, meaning it may contain random data.  Every
    /// bit must be explicitly written to avoid the data containing pre-existing values.
    pub fn spare_capacity_mut(&mut self) -> &mut BitSlice {
        // If the last "in-use" bit is not on a byte boundary, then `self.inner.spare_capacity_mut`
        // will start in the _next_ byte compared to what we actually want, so we can't rely on
        // `self.inner.spare_capacity_mut` along to get us the right slice.

        // The index of the first unused bit, relative to the start of the view
        let bit_start = self.bit_start + self.bit_len;

        // Get the MaybeUninit<u8> spare region
        let spare_uninit = self.inner.spare_capacity_mut();

        // Check the alignment of the first-unused-bit index.  If it's byte-aligned, then the slice
        // we got back from spare_capacity_mut will work as-is.  If it's not, we'll need to
        // decrement it by one byte so that the slice we return starts at the first unused bit.
        let (ptr, len) = if bit_start % 8 == 0 {
            (spare_uninit.as_mut_ptr() as *mut u8, spare_uninit.len())
        } else {
            let ptr = unsafe { spare_uninit.as_mut_ptr().offset(-1) as *mut u8 };
            // Need to add one to the length here to accommodate the byte we "added"
            (ptr, spare_uninit.len() + 1)
        };

        let spare_bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };

        // Create the bitslice from the correct range
        &mut BitSlice::from_slice_mut(spare_bytes)[bit_start % 8..]
    }

    /// Sets the length of the buffer in bits.
    ///
    /// This will explicitly set the size of the buffer without actually modifying the data, so it
    /// is up to the caller to ensure that the data has been initialized.
    pub fn set_len(&mut self, len: usize) {
        self.bit_len = len;
        unsafe { self.inner.set_len(bytes_needed(len)) };
    }

    /// Reserves capacity for at least additional more bits to be inserted into the given
    /// `BitsMut`.
    pub fn reserve(&mut self, additional: usize) {
        let len = self.len();
        let remainder = self.capacity - len;

        if additional <= remainder {
            return;
        }
        let bytes_needed = bytes_needed(additional);
        self.inner.reserve(bytes_needed);
        self.capacity = self.inner.capacity() * 8;
    }

    /// Splits the buffer into two at the given index.
    ///
    /// Afterwards `self` contains elements `[at, len)`, and the returned `BitsMut` contains
    /// elements `[0, at)`.
    pub fn split_to(&mut self, at: usize) -> Self {
        assert!(
            at <= self.bit_len,
            "split_to out of bounds: {:?} must be <= {:?}",
            at,
            self.bit_len
        );

        let mut other = self.clone();
        self.advance_unchecked(at);
        other.capacity = at;
        other.bit_len = at;
        other
    }

    /// Splits the bits into two at the given byte index.  Note that this byte index is relative to
    /// the start of this view, and may not fall on a byte boundary in the underlying storage.
    ///
    /// Afterwards self contains elements [at, len), and the returned BitsMut contains elements [0,
    /// at).
    pub fn split_to_bytes(&mut self, at: usize) -> Self {
        self.split_to(at * 8)
    }

    /// Removes the bits from the current view, returning them in a new `BitsMut` handle.
    ///
    /// Afterwards, self will be empty, but will retain any additional capacity that it had before
    /// the operation. This is identical to self.split_to(self.len()).
    pub fn split(&mut self) -> Self {
        self.split_to(self.bit_len)
    }

    /// Splits the bits into two at the given index.
    ///
    /// Afterwards `self` contains elements `[0, at)`, and the returned `BitsMut`` contains
    /// elements `[at, capacity)`.
    pub fn split_off(&mut self, at: usize) -> Self {
        assert!(
            at <= self.capacity,
            "split_off out of bounds: {:?} must be <= {:?}",
            at,
            self.bit_len
        );

        let mut other = self.clone();
        // Safety: We've checked at <= self.capacity
        other.advance_unchecked(at);
        self.capacity = at;
        self.bit_len = std::cmp::min(self.bit_len, at);

        other
    }

    /// Splits the bits into two at the given byte index.  Note that this byte index is relative to
    /// the start of this view, and may not fall on a byte boundary in the underlying storage.
    ///
    /// Afterwards `self` contains elements `[0, at)`, and the returned `BitsMut` contains
    /// elements `[at, capacity)`.
    pub fn split_off_bytes(&mut self, at: usize) -> Self {
        self.split_off(at * 8)
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

    /// Advance the buffer by `count` bits without bounds checking
    fn advance_unchecked(&mut self, count: usize) {
        if count == 0 {
            return;
        }

        self.bit_start += count;
        self.bit_len = self.bit_len.saturating_sub(count);
        self.capacity -= count;
    }
}

impl Default for BitsMut {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BitVec> for BitsMut {
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
            inner: BytesMut::from(&bytes[..]),
            bit_start: 0,
            bit_len,
            capacity: bytes.len() * 8,
        }
    }
}

impl From<&BitSlice> for BitsMut {
    fn from(slice: &BitSlice) -> Self {
        BitsMut::from(slice.to_bitvec())
    }
}

impl From<Vec<u8>> for BitsMut {
    fn from(vec: Vec<u8>) -> Self {
        let bit_len = vec.len() * 8;
        // Creating a Bytes from the Vec first does so without copying, so we can avoid the copy
        // here
        let inner = BytesMut::from(Bytes::from(vec));
        let byte_capacity = inner.capacity();
        Self {
            inner,
            bit_start: 0,
            bit_len,
            capacity: byte_capacity * 8,
        }
    }
}

impl Deref for BitsMut {
    type Target = BitSlice;

    fn deref(&self) -> &Self::Target {
        &BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len]
    }
}

impl DerefMut for BitsMut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut BitSlice::from_slice_mut(&mut self.inner)
            [self.bit_start..self.bit_start + self.bit_len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_to() {
        let mut bits = BitsMut::from(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        let mut head = bits.split_to(4);
        head.set(0, false);
        head.set(1, false);
        assert_eq!(head[..], bits![0, 0, 1, 1]);

        bits.set(0, true);
        bits.set(1, true);
        assert_eq!(bits[..], bits![1, 1, 0, 0]);
    }

    #[test]
    fn test_split_to_bytes() {
        #[rustfmt::skip]
        let mut bits = BitsMut::from(vec![
            0b1111_1111,
            0b0000_0000,
            0b1010_1010,
            0b0101_0101
        ]);

        let mut head = bits.split_to_bytes(1);
        // 'head' is now bits [0, 8), 'bits' is [8, 32)
        assert_eq!(head.len(), 8);
        assert_eq!(bits.len(), 24);
        head.set(0, false);
        head.set(1, false);
        head.set(2, false);
        head.set(3, false);

        bits.set(0, true);
        bits.set(1, true);
        bits.set(2, true);
        bits.set(3, true);
        assert_eq!(head[..], bits![0, 0, 0, 0, 1, 1, 1, 1]);
        assert_eq!(
            bits[..],
            bits![1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1]
        );
        // Now split at a non-byte boundary and then do a byte-split to make sure that works
        // correctly
        let mut unaligned_split = bits.split_to(12);
        // 'bits' is now bits [20, 32), 'unaligned_split' is [8, 20)
        let mut unaligned_byte_split = unaligned_split.split_to_bytes(1);
        // 'unaligned_split' is now bits [16, 20), 'unaligned_byte_split' is [8, 16)
        assert_eq!(unaligned_byte_split.len(), 8);
        assert_eq!(unaligned_split.len(), 4);

        unaligned_byte_split.set(0, false);
        unaligned_byte_split.set(1, false);
        assert_eq!(unaligned_byte_split[..], bits![0, 0, 1, 1, 0, 0, 0, 0]);

        unaligned_split.set(0, false);
        unaligned_split.set(1, true);
        assert_eq!(unaligned_split[..], bits![0, 1, 1, 0]);
    }

    #[test]
    fn test_split_off() {
        let mut bits = BitsMut::zeroed(32);

        let mut tail = bits.split_off(12);
        assert_eq!(bits.len(), 12);
        assert_eq!(tail.len(), 20);
        bits.set(0, true);
        bits.set(1, true);
        bits.set(2, true);
        bits.set(3, true);
        assert_eq!(bits[..], bits![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0]);

        tail.set(0, true);
        tail.set(1, true);
        tail.set(2, true);
        tail.set(3, true);
        assert_eq!(
            tail[..],
            bits![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_spare_capacity_mut() {
        let mut bits_mut = BitsMut::with_capacity(24);
        let spare = bits_mut.spare_capacity_mut();
        spare.set(0, true);
        bits_mut.set_len(1);

        let spare = bits_mut.spare_capacity_mut();
        spare.set(0, false);
        spare.set(1, false);
        spare.set(2, true);
        bits_mut.set_len(4);

        assert_eq!(&bits_mut[..], bits![1, 0, 0, 1]);
    }

    #[test]
    fn test_extend_from_slice() {
        let mut bits_mut = BitsMut::new();
        let data = bits![0, 1, 1, 0, 1, 1, 0];

        bits_mut.extend_from_slice(data);
        assert_eq!(&bits_mut[..], data);
    }
}
