use bitvec::view::BitView;

use crate::prelude::*;

impl<T: BitBuf + ?Sized> BitBuf for &mut T {
    fn advance_bits(&mut self, count: usize) {
        (**self).advance_bits(count);
    }

    fn remaining_bits(&self) -> usize {
        (**self).remaining_bits()
    }

    fn chunk_bits(&self) -> &BitSlice {
        (**self).chunk_bits()
    }

    fn chunk_bytes(&self) -> &[u8] {
        (**self).chunk_bytes()
    }

    fn byte_aligned(&self) -> bool {
        (**self).byte_aligned()
    }
}

impl BitBuf for Bits {
    fn advance_bits(&mut self, count: usize) {
        assert!(count <= self.remaining_bits(), "advance past end of Bits");
        self.inc_start_bits(count);
    }

    fn remaining_bits(&self) -> usize {
        self.bit_len
    }

    fn chunk_bits(&self) -> &BitSlice {
        &BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.bit_start % 8 == 0);
        assert!(self.bit_len % 8 == 0);

        let byte_start = self.bit_start / 8;
        let size_bytes = self.bit_len / 8;

        &self.inner[byte_start..byte_start + size_bytes]
    }

    fn byte_aligned(&self) -> bool {
        self.bit_start % 8 == 0 && self.bit_len % 8 == 0
    }
}

impl BitBuf for BitsMut {
    fn advance_bits(&mut self, count: usize) {
        assert!(
            count <= self.remaining_bits(),
            "advance past end of BitsMut"
        );
        self.bit_start += count;
        self.bit_len -= count;
        self.capacity -= count;
    }

    fn remaining_bits(&self) -> usize {
        self.len_bits()
    }

    fn chunk_bits(&self) -> &BitSlice {
        &BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());

        let byte_start = self.bit_start / 8;

        &self.inner[byte_start..]
    }

    fn byte_aligned(&self) -> bool {
        self.bit_start % 8 == 0 && self.bit_len % 8 == 0
    }
}

impl BitBuf for &[u8] {
    fn advance_bits(&mut self, count: usize) {
        if self.len() < count {
            panic!("Can't advance past the end of slice");
        }
        *self = &self[count..];
    }

    fn remaining_bits(&self) -> usize {
        self.len() * 8
    }

    fn chunk_bits(&self) -> &BitSlice {
        self[..].view_bits()
    }

    fn chunk_bytes(&self) -> &[u8] {
        self
    }

    fn byte_aligned(&self) -> bool {
        true
    }
}

// TODO: I think we're gonna get bit by not supporting BitSlice<O> here, but come back to that
// later--hopefully we don't need a generic on the trait
// impl BitBuf for &BitSlice {
impl BitBuf for &BitSlice {
    fn advance_bits(&mut self, count: usize) {
        if self.len() < count {
            panic!("Can't advance past end of BitSlice");
        }
        *self = &self[count..];
    }

    fn remaining_bits(&self) -> usize {
        self.len()
    }

    fn chunk_bits(&self) -> &BitSlice {
        self
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());
        let bitvec::domain::Domain::Region { body, .. } = self.domain() else {
            unreachable!("Verified by the assert above");
        };

        body
    }

    fn byte_aligned(&self) -> bool {
        matches!(
            self.domain(),
            bitvec::domain::Domain::Region {
                head: None,
                tail: None,
                ..
            }
        )
    }
}

impl<T: AsRef<BitSlice>> BitBuf for BitCursor<T> {
    fn advance_bits(&mut self, count: usize) {
        let len = self.get_ref().as_ref().len();
        let pos = self.position();

        let max_count = len.saturating_sub(pos as usize);
        if count > max_count {
            panic!("Can't advance beyond end of buffer");
        }
        self.set_position(pos + count as u64);
    }

    fn remaining_bits(&self) -> usize {
        self.get_ref()
            .as_ref()
            .len()
            .saturating_sub(self.position() as usize)
    }

    fn chunk_bits(&self) -> &BitSlice {
        let slice = self.get_ref().as_ref();
        let start = slice.len().min(self.position() as usize);
        &slice[start..]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.byte_aligned());
        let bitslice = self.get_ref().as_ref();
        let bitvec::domain::Domain::Region { body, .. } = bitslice.domain() else {
            unreachable!("Verified by the assert above");
        };

        body
    }

    fn byte_aligned(&self) -> bool {
        // TODO: helper func on BitSlice?
        matches!(
            self.get_ref().as_ref().domain(),
            bitvec::domain::Domain::Region {
                head: None,
                tail: None,
                ..
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write a set of common tests that take B: BitBuf and then run them with different
    // types that impl BitBuf.

    #[test]
    fn test_byte_aligned() {
        // Exactly one byte worth of bits
        let bits = bits![0; 8];
        assert!(bits.byte_aligned());
        // Bits within one byte but not the entire byte shouldn't be considered byte-aligned
        let bits = bits![1, 1, 1];
        assert!(!bits.byte_aligned());
        // 2 bytes worth of bits should be considered byte-aligned
        let bits = bits![0; 16];
        assert!(bits.byte_aligned());
        // 1 byte's worth but not at the start shouldn't be considered byte-aligned
        let bits = bits![0; 9];
        let slice = &bits[1..];
        assert_eq!(8, slice.len());
        assert!(!slice.byte_aligned());
    }

    #[test]
    fn test_bit_buf_bits_advance() {
        let mut bits = Bits::copy_from_bit_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        bits.advance_bits(4);
        assert_eq!(bits.len_bits(), 4);
        assert_eq!(bits.chunk_bits(), bits![0, 0, 0, 0]);
    }

    #[test]
    fn test_bit_buf_bits_mut_advance() {
        let mut bits_mut = BitsMut::zeroed_bits(16);
        bits_mut.advance_bits(8);
        assert_eq!(bits_mut.len_bits(), 8);
    }

    #[test]
    fn test_bits_copy_to_slice() {
        let mut bits = Bits::copy_from_bit_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        let dest = bits![mut 0; 4];
        bits.copy_to_bit_slice(dest);
        assert_eq!(dest, bits![1, 1, 1, 1,]);

        bits.copy_to_bit_slice(dest);
        assert_eq!(dest, bits![0, 0, 0, 0]);
    }

    #[test]
    fn test_chunk_bytes() {
        {
            let bits = Bits::copy_from_bit_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

            let chunk_bytes = bits.chunk_bytes();
            assert_eq!(chunk_bytes.len(), 1);
            assert_eq!(chunk_bytes[0], 0b11110000);
        }
        {
            let mut bits = Bits::copy_from_bit_slice(bits![
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0
            ]);
            bits.advance_bits(8);
            let chunk_bytes = bits.chunk_bytes();
            assert_eq!(chunk_bytes.len(), 2);
            assert_eq!(chunk_bytes, [0b11111111, 0b10101010]);
        }
    }

    #[test]
    fn test_chunk_after_split() {
        // Make sure that a call to chunk after some kind of split respects the new limit
        let mut bits = Bits::from_static_bytes(&[1, 2, 3, 4, 5]);

        let start = bits.split_to_bytes(2);
        let start_chunk = start.chunk_bytes();
        assert_eq!(start_chunk.len(), 2);
    }

    #[test]
    fn test_copy_to_slice_bytes() {
        let mut dest = [0; 4];

        let mut bits = Bits::from_owner_bytes([42, 43, 44, 45]);

        bits.copy_to_slice_bytes(&mut dest);
        assert_eq!(dest, [42, 43, 44, 45]);
    }

    #[test]
    fn test_bitslice_bitbuf() {
        let mut bits = bits![1, 0, 1, 0, 1, 0];
        assert_eq!(6, bits.remaining_bits());
        bits.advance_bits(3);
        assert_eq!(3, bits.remaining_bits());
    }

    #[test]
    fn test_take() {
        let mut bits = Bits::from_static_bytes(&[1, 2, 3, 4]);

        let mut head = (&mut bits).take_bits(16);
        let value = head.get_u16::<NetworkOrder>().unwrap();
        assert!(head.get_bool().is_err());
        assert_eq!(value, 0x0102);
        let mut tail = (&mut bits).take_bits(16);
        let value = tail.get_u16::<NetworkOrder>().unwrap();
        assert!(tail.get_bool().is_err());
        assert_eq!(value, 0x0304);
    }
}
