use crate::prelude::*;

/// A trait for objects which are bit-oriented sinks.
pub trait BitWrite: std::io::Write {
    /// Write a buffer into this writer, returning how many bytes were written.
    fn write_bits<O: BitStore>(&mut self, source: &BitSlice<O>) -> std::io::Result<usize>;

    /// Write the entirety buf into self.
    fn write_all_bits<O: BitStore>(&mut self, mut source: &BitSlice<O>) -> std::io::Result<()> {
        while !source.is_empty() {
            let n = self.write_bits(source)?;
            if n == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write all bits",
                ));
            }
            source = &source[n..];
        }
        Ok(())
    }
}
