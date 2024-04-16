use ux::u1;

pub trait BitRead {
    /// Pull some bytes from this source into the specified buffer, returning how many bytes were read.
    fn read(&mut self, buf: &mut [u1]) -> std::io::Result<usize>;

    /// Read the exact number of bytes required to fill buf.
    fn read_exact(&mut self, buf: &mut [u1]) -> std::io::Result<()> {
        read_exact_helper(self, buf)
    }
}

fn read_exact_helper<R: BitRead + ?Sized>(this: &mut R, mut buf: &mut [u1]) -> std::io::Result<()> {
    while !buf.is_empty() {
        // Note: unlike std::io::Read, we don't have a special case for an 'interrupted'
        // error, since we don't have access to all the error data it uses.
        // TODO: look into if we can replicate the is_interrupted logic here somehow.
        match this.read(buf) {
            Ok(0) => break,
            Ok(n) => buf = &mut buf[n..],
            Err(e) => return Err(e),
        }
    }
    if !buf.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "failed to fill whole buffer",
        ))
    } else {
        Ok(())
    }
}
