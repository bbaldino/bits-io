use std::ops::{BitOrAssign, ShlAssign};

use ux::*;

use crate::{bit_read::BitRead, byte_order::ByteOrder};

fn bit_read_exts_helper<
    T: Default + ShlAssign<u32> + BitOrAssign + From<u1>,
    const N: usize,
    U: BitRead + ?Sized,
>(
    buf: &mut U,
) -> std::io::Result<T> {
    // TODO: it'd be nice not to do this bit-by-bit.  I think once we get the from_xxx_bytes methods
    // in ux those could work here.
    let mut read_buf = [u1::default(); N];
    buf.read_exact(&mut read_buf)?;
    let mut val = T::default();
    for bit in read_buf.iter() {
        val <<= 1;
        val |= (*bit).into();
    }
    Ok(val)
}

pub trait BitReadExts: BitRead {
    fn read_bool(&mut self) -> std::io::Result<bool> {
        self.read_u1().map(|v| v.into())
    }

    fn read_u1(&mut self) -> std::io::Result<u1> {
        bit_read_exts_helper::<u1, 1, Self>(self)
    }

    fn read_u2(&mut self) -> std::io::Result<u2> {
        bit_read_exts_helper::<u2, 2, Self>(self)
    }

    fn read_u3(&mut self) -> std::io::Result<u3> {
        bit_read_exts_helper::<u3, 3, Self>(self)
    }

    fn read_u4(&mut self) -> std::io::Result<u4> {
        bit_read_exts_helper::<u4, 4, Self>(self)
    }

    fn read_u5(&mut self) -> std::io::Result<u5> {
        bit_read_exts_helper::<u5, 5, Self>(self)
    }

    fn read_u6(&mut self) -> std::io::Result<u6> {
        bit_read_exts_helper::<u6, 6, Self>(self)
    }

    fn read_u7(&mut self) -> std::io::Result<u7> {
        bit_read_exts_helper::<u7, 7, Self>(self)
    }

    fn read_u8(&mut self) -> std::io::Result<u8> {
        bit_read_exts_helper::<u8, 8, Self>(self)
    }

    fn read_u9<T: ByteOrder>(&mut self) -> std::io::Result<u9> {
        let mut buf = [u1::new(0); 9];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u9(&buf))
    }

    fn read_u10<T: ByteOrder>(&mut self) -> std::io::Result<u10> {
        let mut buf = [u1::new(0); 10];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u10(&buf))
    }

    fn read_u11<T: ByteOrder>(&mut self) -> std::io::Result<u11> {
        let mut buf = [u1::new(0); 11];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u11(&buf))
    }

    fn read_u12<T: ByteOrder>(&mut self) -> std::io::Result<u12> {
        let mut buf = [u1::new(0); 12];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u12(&buf))
    }

    fn read_u13<T: ByteOrder>(&mut self) -> std::io::Result<u13> {
        let mut buf = [u1::new(0); 13];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u13(&buf))
    }

    fn read_u14<T: ByteOrder>(&mut self) -> std::io::Result<u14> {
        let mut buf = [u1::new(0); 14];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u14(&buf))
    }

    fn read_u15<T: ByteOrder>(&mut self) -> std::io::Result<u15> {
        let mut buf = [u1::new(0); 15];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u15(&buf))
    }

    fn read_u16<T: ByteOrder>(&mut self) -> std::io::Result<u16> {
        let mut buf = [u1::new(0); 16];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u16(&buf))
    }

    fn read_u17<T: ByteOrder>(&mut self) -> std::io::Result<u17> {
        let mut buf = [u1::new(0); 17];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u17(&buf))
    }

    fn read_u18<T: ByteOrder>(&mut self) -> std::io::Result<u18> {
        let mut buf = [u1::new(0); 18];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u18(&buf))
    }

    fn read_u19<T: ByteOrder>(&mut self) -> std::io::Result<u19> {
        let mut buf = [u1::new(0); 19];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u19(&buf))
    }

    fn read_u20<T: ByteOrder>(&mut self) -> std::io::Result<u20> {
        let mut buf = [u1::new(0); 20];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u20(&buf))
    }

    fn read_u21<T: ByteOrder>(&mut self) -> std::io::Result<u21> {
        let mut buf = [u1::new(0); 21];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u21(&buf))
    }

    fn read_u22<T: ByteOrder>(&mut self) -> std::io::Result<u22> {
        let mut buf = [u1::new(0); 22];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u22(&buf))
    }

    fn read_u23<T: ByteOrder>(&mut self) -> std::io::Result<u23> {
        let mut buf = [u1::new(0); 23];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u23(&buf))
    }

    fn read_u24<T: ByteOrder>(&mut self) -> std::io::Result<u24> {
        let mut buf = [u1::new(0); 24];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u24(&buf))
    }

    fn read_u25<T: ByteOrder>(&mut self) -> std::io::Result<u25> {
        let mut buf = [u1::new(0); 25];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u25(&buf))
    }

    fn read_u26<T: ByteOrder>(&mut self) -> std::io::Result<u26> {
        let mut buf = [u1::new(0); 26];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u26(&buf))
    }

    fn read_u27<T: ByteOrder>(&mut self) -> std::io::Result<u27> {
        let mut buf = [u1::new(0); 27];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u27(&buf))
    }

    fn read_u28<T: ByteOrder>(&mut self) -> std::io::Result<u28> {
        let mut buf = [u1::new(0); 28];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u28(&buf))
    }

    fn read_u29<T: ByteOrder>(&mut self) -> std::io::Result<u29> {
        let mut buf = [u1::new(0); 29];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u29(&buf))
    }

    fn read_u30<T: ByteOrder>(&mut self) -> std::io::Result<u30> {
        let mut buf = [u1::new(0); 30];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u30(&buf))
    }

    fn read_u31<T: ByteOrder>(&mut self) -> std::io::Result<u31> {
        let mut buf = [u1::new(0); 31];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u31(&buf))
    }

    fn read_u32<T: ByteOrder>(&mut self) -> std::io::Result<u32> {
        let mut buf = [u1::new(0); 32];
        self.read_exact(&mut buf)?;
        Ok(<T>::read_u32(&buf))
    }
}

impl<T> BitReadExts for T where T: BitRead {}

#[cfg(test)]
mod test {
    use ux::u1;

    use super::BitReadExts;
    use crate::bit_cursor::BitCursor;

    #[test]
    fn test_read() {
        let data: Vec<u8> = vec![0b10000000];
        let mut cursor = BitCursor::new(data);

        assert_eq!(cursor.read_u1().unwrap(), u1::new(1));
    }
}
