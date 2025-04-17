use super::{bit_buf::BitBuf, bits::Bits};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_buf_advance() {
        let mut bits = Bits::copy_from_slice(bits![1, 1, 1, 1, 0, 0, 0, 0]);

        bits.advance(4);
        assert_eq!(bits.chunk(), bits![0, 0, 0, 0]);
    }
}
