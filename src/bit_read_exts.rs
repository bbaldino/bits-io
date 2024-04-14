use std::ops::{BitOrAssign, ShlAssign};

use ux::u1;

use crate::bit_read::BitRead;

fn bit_read_exts_helper<
    T: Default + ShlAssign<u32> + BitOrAssign<u1>,
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
        val |= *bit;
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
