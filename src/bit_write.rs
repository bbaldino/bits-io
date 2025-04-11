use crate::prelude::*;

pub trait BitWrite: std::io::Write {
    /// Write a buffer into this writer, returning how many bytes were written.
    fn write_bits<O: BitStore>(&mut self, buf: &BitSlice<O>) -> std::io::Result<usize>;

    /// Write the entirety buf into self.
    /// TODO: rename 'buf' to 'source'
    fn write_all_bits<O: BitStore>(&mut self, mut buf: &BitSlice<O>) -> std::io::Result<()> {
        while !buf.is_empty() {
            let n = self.write_bits(buf)?;
            if n == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write all bits",
                ));
            }
            buf = &buf[n..];
        }
        Ok(())
    }
}
