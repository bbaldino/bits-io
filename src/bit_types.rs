pub trait BitStore: bitvec::store::BitStore {}

impl BitStore for u8 {}
impl BitStore for bitvec::access::BitSafeU8 {}

pub type BitSlice<O = u8> = bitvec::slice::BitSlice<O, bitvec::order::Msb0>;

pub type BitVec = bitvec::vec::BitVec<u8, bitvec::order::Msb0>;
