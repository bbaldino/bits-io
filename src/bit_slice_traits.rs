use bitvec::{field::BitField, order::BitOrder, slice::BitSlice, store::BitStore};
use ux::u1;

use crate::{bit_read::BitRead, bit_write::BitWrite};

impl<T, O> BitRead for &BitSlice<T, O>
where
    T: BitStore,
    O: BitOrder,
    BitSlice<T, O>: BitField,
{
    fn read_bits(&mut self, buf: &mut [u1]) -> std::io::Result<usize> {
        let n = self.len().min(buf.len());
        for (i, bit) in buf.iter_mut().enumerate().take(n) {
            *bit = self[i].into()
        }
        Ok(n)
    }
}

impl<T, O> BitWrite for &mut BitSlice<T, O>
where
    T: BitStore,
    O: BitOrder,
    BitSlice<T, O>: BitField,
{
    fn write_bits(&mut self, buf: &[u1]) -> std::io::Result<usize> {
        let n = self.len().min(buf.len());
        for (i, bit) in buf.iter().enumerate().take(n) {
            self.set(i, (*bit).into())
        }
        Ok(n)
    }
}
