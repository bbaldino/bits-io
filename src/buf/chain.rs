use crate::prelude::BitBufMut;

use super::bit_buf::BitBuf;

/// A `Chain` sequences to `BitBuf`s.
///
/// `Chain` is an adaptor that links to underlying buffers and provides a continuous view across
/// both.
pub struct Chain<T, U> {
    a: T,
    b: U,
}

impl<T, U> Chain<T, U> {
    pub fn new(a: T, b: U) -> Self {
        Self { a, b }
    }

    pub fn first_ref(&self) -> &T {
        &self.a
    }

    pub fn last_ref(&self) -> &U {
        &self.b
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.a
    }

    pub fn last_mut(&mut self) -> &mut U {
        &mut self.b
    }

    pub fn into_inner(self) -> (T, U) {
        (self.a, self.b)
    }
}

impl<T, U> BitBuf for Chain<T, U>
where
    T: BitBuf,
    U: BitBuf,
{
    fn advance_bits(&mut self, mut count: usize) {
        let a_rem = self.a.remaining_bits();

        if a_rem != 0 {
            if a_rem >= count {
                self.a.advance_bits(count);
                return;
            }

            // Consume what is left of a
            self.a.advance_bits(a_rem);

            count -= a_rem;
        }
        self.b.advance_bits(count);
    }

    fn remaining_bits(&self) -> usize {
        self.a
            .remaining_bits()
            .saturating_add(self.b.remaining_bits())
    }

    fn chunk_bits(&self) -> &crate::prelude::BitSlice {
        if self.a.has_remaining_bits() {
            self.a.chunk_bits()
        } else {
            self.b.chunk_bits()
        }
    }

    fn chunk_bytes(&self) -> &[u8] {
        if self.a.has_remaining_bytes() {
            self.a.chunk_bytes()
        } else {
            self.b.chunk_bytes()
        }
    }

    fn byte_aligned(&self) -> bool {
        self.a.byte_aligned() && self.b.byte_aligned()
    }
}

impl<T, U> BitBufMut for Chain<T, U>
where
    T: BitBufMut,
    U: BitBufMut,
{
    fn advance_mut_bits(&mut self, mut count: usize) {
        let a_rem = self.a.remaining_mut_bits();

        if a_rem != 0 {
            if a_rem >= count {
                self.a.advance_mut_bits(count);
                return;
            }

            // Consume what's left in a
            self.a.advance_mut_bits(a_rem);

            count -= a_rem;
        }
        self.b.advance_mut_bits(count);
    }

    fn chunk_mut_bits(&mut self) -> &mut crate::prelude::BitSlice {
        if self.a.has_remaining_mut_bits() {
            self.a.chunk_mut_bits()
        } else {
            self.b.chunk_mut_bits()
        }
    }

    fn chunk_mut_bytes(&mut self) -> &mut bytes::buf::UninitSlice {
        if self.a.has_reminaing_mut_bytes() {
            self.a.chunk_mut_bytes()
        } else {
            self.b.chunk_mut_bytes()
        }
    }

    fn remaining_mut_bits(&self) -> usize {
        self.a
            .remaining_mut_bits()
            .saturating_add(self.b.remaining_mut_bits())
    }

    fn byte_aligned_mut(&self) -> bool {
        self.a.byte_aligned_mut() && self.b.byte_aligned_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    #[test]
    fn test_bit_buf_chain() {
        let left = Bits::from(bits![1, 1, 1, 1, 0, 0, 0, 0]);
        let right = Bits::from(bits![1, 0, 1, 0, 1, 0, 1, 0]);

        let mut chain = left.chain(right);

        let mut data = [0u8; 2];
        chain.copy_to_slice_bytes(&mut data);

        assert_eq!(data, [0b11110000, 0b10101010]);
    }

    #[test]
    fn test_bit_buf_mut_chain() {
        let mut left = [0u8; 1];
        let mut right = [0u8; 1];

        let mut chain = (&mut left[..]).chain_mut(&mut right[..]);
        chain.put_u16::<NetworkOrder>(42).unwrap();
        assert_eq!(&left[..], [0]);
        assert_eq!(&right[..], [42]);
    }
}
