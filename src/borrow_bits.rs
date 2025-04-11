use bitvec::prelude::*;

pub trait BorrowBits {
    fn borrow_bits(&self) -> &BitSlice<u8, Msb0>;
}

impl BorrowBits for Vec<u8> {
    fn borrow_bits(&self) -> &BitSlice<u8, Msb0> {
        self.view_bits::<Msb0>()
    }
}

impl BorrowBits for BitVec<u8, Msb0> {
    fn borrow_bits(&self) -> &BitSlice<u8, Msb0> {
        self.as_bitslice()
    }
}

impl BorrowBits for &[u8] {
    fn borrow_bits(&self) -> &BitSlice<u8, Msb0> {
        self.view_bits::<Msb0>()
    }
}

impl BorrowBits for &BitSlice<u8, Msb0> {
    fn borrow_bits(&self) -> &BitSlice<u8, Msb0> {
        self
    }
}

impl BorrowBits for &mut [u8] {
    fn borrow_bits(&self) -> &BitSlice<u8, Msb0> {
        self.view_bits::<Msb0>()
    }
}

impl BorrowBits for &mut BitSlice<u8, Msb0> {
    fn borrow_bits(&self) -> &BitSlice<u8, Msb0> {
        self
    }
}

pub trait BorrowBitsMut: BorrowBits {
    fn borrow_bits_mut(&mut self) -> &mut BitSlice<u8, Msb0>;
}

impl BorrowBitsMut for Vec<u8> {
    fn borrow_bits_mut(&mut self) -> &mut BitSlice<u8, Msb0> {
        self.view_bits_mut::<Msb0>()
    }
}

impl BorrowBitsMut for BitVec<u8, Msb0> {
    fn borrow_bits_mut(&mut self) -> &mut BitSlice<u8, Msb0> {
        self.as_mut_bitslice()
    }
}

impl BorrowBitsMut for &mut [u8] {
    fn borrow_bits_mut(&mut self) -> &mut BitSlice<u8, Msb0> {
        self.view_bits_mut::<Msb0>()
    }
}

impl BorrowBitsMut for &mut BitSlice<u8, Msb0> {
    fn borrow_bits_mut(&mut self) -> &mut BitSlice<u8, Msb0> {
        self
    }
}
