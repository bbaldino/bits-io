use crate::prelude::*;

impl BitRead for &BitSlice {
    fn read_bits(&mut self, dest: &mut BitSlice) -> std::io::Result<usize> {
        let n = self.len().min(dest.len());
        dest[..n].copy_from_bitslice(&self[..n]);
        Ok(n)
    }
}

impl<S: BitStore> BitWrite for &mut BitSlice<S> {
    fn write_bits<O: BitStore>(&mut self, source: &BitSlice<O>) -> std::io::Result<usize> {
        let n = self.len().min(source.len());
        self[..n].clone_from_bitslice(&source[..n]);

        *self = &mut std::mem::take(self)[n..];

        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_write_unique_slice() {
        let mut buf = [0u8; 2];
        let mut bits = BitSlice::from_slice_mut(&mut buf[..]);

        bits.write_bits(bits![1]).unwrap();
        bits.write_bits(bits![0, 0, 1]).unwrap();
        bits.write_bits(bits![0, 0, 0, 0, 1]).unwrap();

        assert_eq!(buf, [0b10010000, 0b10000000]);
    }

    #[test]
    fn test_bit_write_shared_slice() {
        let mut buf = [0u8; 2];
        let bits = BitSlice::from_slice_mut(&mut buf[..]);
        let (mut left, mut right) = bits.split_at_mut(8);

        left.write_bits(bits![1]).unwrap();
        left.write_bits(bits![0, 0, 1]).unwrap();

        right.write_bits(bits![0, 1]).unwrap();
        right.write_bits(bits![0, 0, 0, 1]).unwrap();

        assert_eq!(buf, [0b10010000, 0b01000100]);
    }
}
