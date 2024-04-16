use std::ops::{BitAnd, ShrAssign};

use crate::{bit_write::BitWrite, byte_order::ByteOrder};
use num_traits::{ConstOne, ConstZero};
use ux::*;

fn bit_write_exts_helper<T, const N: usize, U>(buf: &mut U, mut value: T) -> std::io::Result<()>
where
    T: ConstOne + BitAnd<Output = T> + Copy + ShrAssign<u32> + Eq,
    U: BitWrite + ?Sized,
{
    let mut arr = [u1::ZERO; N];
    let index_offset = N - 1;
    for i in 0..N {
        let lsb = value & T::ONE;
        let bit = if lsb == T::ONE { u1::ONE } else { u1::ZERO };
        arr[index_offset - i] = bit;
        value >>= 1;
    }
    buf.write_all(&arr)
}

/// A trait which extends BitWrite to add explicit write methods for non-standard-width types.
pub trait BitWriteExts: BitWrite {
    fn write_bool(&mut self, value: bool) -> std::io::Result<()> {
        self.write_u1(value.into())
    }

    fn write_u1(&mut self, value: u1) -> std::io::Result<()> {
        self.write_all(&[value])
    }

    fn write_u2(&mut self, value: u2) -> std::io::Result<()> {
        bit_write_exts_helper::<u2, 2, Self>(self, value)
    }

    fn write_u3(&mut self, value: u3) -> std::io::Result<()> {
        bit_write_exts_helper::<u3, 3, Self>(self, value)
    }

    fn write_u4(&mut self, value: u4) -> std::io::Result<()> {
        bit_write_exts_helper::<u4, 4, Self>(self, value)
    }

    fn write_u5(&mut self, value: u5) -> std::io::Result<()> {
        bit_write_exts_helper::<u5, 5, Self>(self, value)
    }

    fn write_u6(&mut self, value: u6) -> std::io::Result<()> {
        bit_write_exts_helper::<u6, 6, Self>(self, value)
    }

    fn write_u7(&mut self, value: u7) -> std::io::Result<()> {
        bit_write_exts_helper::<u7, 7, Self>(self, value)
    }

    fn write_u8(&mut self, value: u8) -> std::io::Result<()> {
        bit_write_exts_helper::<u8, 8, Self>(self, value)
    }

    fn write_u9<T: ByteOrder>(&mut self, value: u9) -> std::io::Result<()> {
        let mut arr = [u1::default(); 9];
        T::write_u9(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u10<T: ByteOrder>(&mut self, value: u10) -> std::io::Result<()> {
        let mut arr = [u1::default(); 10];
        T::write_u10(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u11<T: ByteOrder>(&mut self, value: u11) -> std::io::Result<()> {
        let mut arr = [u1::default(); 11];
        T::write_u11(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u12<T: ByteOrder>(&mut self, value: u12) -> std::io::Result<()> {
        let mut arr = [u1::default(); 12];
        T::write_u12(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u13<T: ByteOrder>(&mut self, value: u13) -> std::io::Result<()> {
        let mut arr = [u1::default(); 13];
        T::write_u13(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u14<T: ByteOrder>(&mut self, value: u14) -> std::io::Result<()> {
        let mut arr = [u1::default(); 14];
        T::write_u14(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u15<T: ByteOrder>(&mut self, value: u15) -> std::io::Result<()> {
        let mut arr = [u1::default(); 15];
        T::write_u15(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u16<T: ByteOrder>(&mut self, value: u16) -> std::io::Result<()> {
        let mut arr = [u1::default(); 16];
        T::write_u16(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u17<T: ByteOrder>(&mut self, value: u17) -> std::io::Result<()> {
        let mut arr = [u1::default(); 17];
        T::write_u17(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u18<T: ByteOrder>(&mut self, value: u18) -> std::io::Result<()> {
        let mut arr = [u1::default(); 18];
        T::write_u18(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u19<T: ByteOrder>(&mut self, value: u19) -> std::io::Result<()> {
        let mut arr = [u1::default(); 19];
        T::write_u19(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u20<T: ByteOrder>(&mut self, value: u20) -> std::io::Result<()> {
        let mut arr = [u1::default(); 20];
        T::write_u20(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u21<T: ByteOrder>(&mut self, value: u21) -> std::io::Result<()> {
        let mut arr = [u1::default(); 21];
        T::write_u21(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u22<T: ByteOrder>(&mut self, value: u22) -> std::io::Result<()> {
        let mut arr = [u1::default(); 22];
        T::write_u22(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u23<T: ByteOrder>(&mut self, value: u23) -> std::io::Result<()> {
        let mut arr = [u1::default(); 23];
        T::write_u23(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u24<T: ByteOrder>(&mut self, value: u24) -> std::io::Result<()> {
        let mut arr = [u1::default(); 24];
        T::write_u24(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u25<T: ByteOrder>(&mut self, value: u25) -> std::io::Result<()> {
        let mut arr = [u1::default(); 25];
        T::write_u25(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u26<T: ByteOrder>(&mut self, value: u26) -> std::io::Result<()> {
        let mut arr = [u1::default(); 26];
        T::write_u26(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u27<T: ByteOrder>(&mut self, value: u27) -> std::io::Result<()> {
        let mut arr = [u1::default(); 27];
        T::write_u27(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u28<T: ByteOrder>(&mut self, value: u28) -> std::io::Result<()> {
        let mut arr = [u1::default(); 28];
        T::write_u28(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u29<T: ByteOrder>(&mut self, value: u29) -> std::io::Result<()> {
        let mut arr = [u1::default(); 29];
        T::write_u29(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u30<T: ByteOrder>(&mut self, value: u30) -> std::io::Result<()> {
        let mut arr = [u1::default(); 30];
        T::write_u30(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u31<T: ByteOrder>(&mut self, value: u31) -> std::io::Result<()> {
        let mut arr = [u1::default(); 31];
        T::write_u31(&mut arr, value);
        self.write_all(&arr)
    }

    fn write_u32<T: ByteOrder>(&mut self, value: u32) -> std::io::Result<()> {
        let mut arr = [u1::default(); 32];
        T::write_u32(&mut arr, value);
        self.write_all(&arr)
    }
}

impl<T> BitWriteExts for T where T: BitWrite {}
