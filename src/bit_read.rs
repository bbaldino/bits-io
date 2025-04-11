use crate::prelude::*;

pub trait BitRead: std::io::Read {
    /// Pull some bits from this source into the specified buffer, returning how many bytes were read.
    fn read_bits(&mut self, dest: &mut BitSlice) -> std::io::Result<usize>;

    /// Read the exact number of bits required to fill buf.
    fn read_bits_exact(&mut self, dest: &mut BitSlice) -> std::io::Result<()> {
        read_bits_exact_helper(self, dest)
    }
}

fn read_bits_exact_helper<R: BitRead + ?Sized>(
    this: &mut R,
    mut dest: &mut BitSlice,
) -> std::io::Result<()> {
    while !dest.is_empty() {
        // Note: unlike std::io::Read, we don't have a special case for an 'interrupted'
        // error, since we don't have access to all the error data it uses.
        // TODO: look into if we can replicate the is_interrupted logic here somehow.
        match this.read_bits(dest) {
            Ok(0) => break,
            Ok(n) => dest = &mut dest[n..],
            Err(e) => return Err(e),
        }
    }
    if !dest.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "failed to fill whole buffer",
        ))
    } else {
        Ok(())
    }
}
