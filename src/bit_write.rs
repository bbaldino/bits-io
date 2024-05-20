use ux::u1;

pub trait BitWrite: std::io::Write {
    /// Write a buffer into this writer, returning how many bytes were written.
    fn write_bits(&mut self, buf: &[u1]) -> std::io::Result<usize>;

    /// Write the entirety buf into self.
    fn write_all_bits(&mut self, mut buf: &[u1]) -> std::io::Result<()> {
        while !buf.is_empty() {
            match self.write_bits(buf) {
                Ok(0) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "failed to write whole buffer",
                    ))
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
            }
        }

        Ok(())
    }
}
