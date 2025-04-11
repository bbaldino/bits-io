use bitvec::{order::Msb0, slice::BitSlice, store::BitStore};

use crate::{bit_read::BitRead, bit_write::BitWrite};

impl BitRead for &BitSlice<u8, Msb0> {
    fn read_bits(&mut self, dest: &mut BitSlice<u8, Msb0>) -> std::io::Result<usize> {
        let n = self.len().min(dest.len());
        dest[..n].copy_from_bitslice(&self[..n]);
        Ok(n)
    }
}

impl<S: BitStore> BitWrite for &mut BitSlice<S, Msb0> {
    fn write_bits<O: BitStore>(&mut self, source: &BitSlice<O, Msb0>) -> std::io::Result<usize> {
        let n = self.len().min(source.len());
        for (i, bit) in source.iter().enumerate().take(n) {
            self.set(i, *bit)
        }
        Ok(n)
    }
}
