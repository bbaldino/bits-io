use bitvec::{order::Msb0, slice::BitSlice};
use ux::u1;

use crate::{bit_read::BitRead, bit_write::BitWrite};

impl BitRead for &BitSlice<u8, Msb0> {
    fn read_bits(&mut self, buf: &mut [u1]) -> std::io::Result<usize> {
        let n = self.len().min(buf.len());
        for (i, bit) in buf.iter_mut().enumerate().take(n) {
            *bit = self[i].into()
        }
        Ok(n)
    }
}

impl BitWrite for &mut BitSlice<u8, Msb0> {
    fn write_bits(&mut self, buf: &[u1]) -> std::io::Result<usize> {
        let n = self.len().min(buf.len());
        for (i, bit) in buf.iter().enumerate().take(n) {
            self.set(i, (*bit).into())
        }
        Ok(n)
    }
}
