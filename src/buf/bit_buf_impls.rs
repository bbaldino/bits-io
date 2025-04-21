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
}
