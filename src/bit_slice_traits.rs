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
        for (i, bit) in source.iter().enumerate().take(n) {
            self.set(i, *bit)
        }
        Ok(n)
    }
}
