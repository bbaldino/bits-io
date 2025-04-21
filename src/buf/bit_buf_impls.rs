use crate::prelude::*;

impl BitBuf for Bits {
    fn advance(&mut self, count: usize) {
        assert!(count <= self.remaining(), "advance past end of Bits");
        self.inc_start(count);
    }

    fn remaining(&self) -> usize {
        self.bit_len
    }

    fn chunk(&self) -> &BitSlice {
        &BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.bit_start % 8 == 0);
        assert!(self.bit_len % 8 == 0);

        let byte_start = self.bit_start / 8;

        &self.inner[byte_start..]
    }

    fn copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) {
        assert!(self.bit_start % 8 == 0);
        assert!(self.bit_len % 8 == 0);
        if self.remaining_bytes() < dest.len() {
            panic!(
                "Remaining bytes ({}) are less than the size of the dest ({})",
                self.remaining_bytes(),
                dest.len()
            );
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance(count);
        }
    }
}

impl BitBuf for BitsMut {
    fn advance(&mut self, count: usize) {
        assert!(count <= self.remaining(), "advance past end of BitsMut");
        self.bit_start += count;
        self.bit_len -= count;
        self.capacity -= count;
    }

    fn remaining(&self) -> usize {
        self.len()
    }

    fn chunk(&self) -> &BitSlice {
        &BitSlice::from_slice(&self.inner)[self.bit_start..self.bit_start + self.bit_len]
    }

    fn chunk_bytes(&self) -> &[u8] {
        assert!(self.bit_start % 8 == 0);
        assert!(self.bit_len % 8 == 0);

        let byte_start = self.bit_start / 8;

        &self.inner[byte_start..]
    }

    fn copy_to_slice_bytes(&mut self, mut dest: &mut [u8]) {
        assert!(self.bit_start % 8 == 0);
        assert!(self.bit_len % 8 == 0);
        if self.remaining_bytes() < dest.len() {
            panic!(
                "Remaining bytes ({}) are less than the size of the dest ({})",
                self.remaining_bytes(),
                dest.len()
            );
        }
        while !dest.is_empty() {
            let src = self.chunk_bytes();
            let count = usize::min(src.len(), dest.len());
            dest[..count].copy_from_slice(&src[..count]);
            dest = &mut dest[count..];

            self.advance(count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_buf_bits_advance() {
        let mut bits = Bits::copy_from_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        bits.advance(4);
        assert_eq!(bits.len(), 4);
        assert_eq!(bits.chunk(), bits![0, 0, 0, 0]);
    }

    #[test]
    fn test_bit_buf_bits_mut_advance() {
        let mut bits_mut = BitsMut::zeroed(16);
        bits_mut.advance(8);
        assert_eq!(bits_mut.len(), 8);
    }

    #[test]
    fn test_bits_copy_to_slice() {
        let mut bits = Bits::copy_from_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        let dest = bits![mut 0; 4];
        bits.copy_to_slice(dest);
        assert_eq!(dest, bits![1, 1, 1, 1,]);

        bits.copy_to_slice(dest);
        assert_eq!(dest, bits![0, 0, 0, 0]);
    }

    #[test]
    fn test_chunk_bytes() {
        {
            let bits = Bits::copy_from_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

            let chunk_bytes = bits.chunk_bytes();
            assert_eq!(chunk_bytes.len(), 1);
            assert_eq!(chunk_bytes[0], 0b11110000);
        }
        {
            let mut bits = Bits::copy_from_slice(bits![
                0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0
            ]);
            bits.advance(8);
            let chunk_bytes = bits.chunk_bytes();
            assert_eq!(chunk_bytes.len(), 2);
            assert_eq!(chunk_bytes, [0b11111111, 0b10101010]);
        }
    }

    #[test]
    fn test_copy_to_slice_bytes() {
        let mut dest = [0; 4];

        let mut bits = Bits::from_owner_bytes([42, 43, 44, 45]);

        bits.copy_to_slice_bytes(&mut dest);
        assert_eq!(dest, [42, 43, 44, 45]);
    }
}
