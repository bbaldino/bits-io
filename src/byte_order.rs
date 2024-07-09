use nsw_types::*;
use paste::paste;

const U1_ONE: u1 = u1::new(1);

// Helper macro to provide consistent syntax for initializing both nsw_types and standard integer types
// TODO: better to just define a trait that implements ONE and ZERO for all types?
macro_rules! init_integer {
    (u16, $value:expr) => {
        $value
    };
    (u32, $value:expr) => {
        $value
    };
    ($type:ty, $value:expr) => {
        <$type>::new($value)
    };
}

/// Generate a LittleEndian read operation from a buffer for the nsw_types types
macro_rules! impl_read_le {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](buf: &[u1; $size_bits]) -> $type {
                let mut val = <$type>::default();
                if $size_bits > 32 {
                    unimplemented!("Only nsw_types types up to 32 bits supported");
                }
                if $size_bits > 24 {
                    for i in 24..std::cmp::min($size_bits, 32) {
                        val <<= 1;
                        val |= <nsw_types::u1 as Into<$type>>::into(buf[i]);
                    }
                }
                if $size_bits > 16 {
                    for i in 16..std::cmp::min($size_bits, 24) {
                        val <<= 1;
                        val |= <nsw_types::u1 as Into<$type>>::into(buf[i]);
                    }
                }
                if $size_bits > 8 {
                    for i in 8..std::cmp::min($size_bits, 16) {
                        val <<= 1;
                        val |= <nsw_types::u1 as Into<$type>>::into(buf[i]);
                    }
                }
                for i in 0..std::cmp::min($size_bits, 8) {
                    val <<= 1;
                    val |= <nsw_types::u1 as Into<$type>>::into(buf[i]);
                }
                val
            }
        }
    };
}

macro_rules! impl_read_be {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<read_ $type>](buf: &[u1; $size_bits]) -> $type {
                let mut val = <$type>::default();
                for bit in buf.iter() {
                    val <<= 1;
                    val |= <nsw_types::u1 as Into<$type>>::into(*bit);
                }
                val
            }
        }
    };
}

macro_rules! impl_write_le {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<write_ $type>](buf: &mut [u1; $size_bits], value: $type) {
                const ZERO: $type = init_integer!($type, 0);
                const ONE: $type = init_integer!($type, 1);
                let mut mask = ONE << std::cmp::min($size_bits - 1, 7);
                for i in 0..std::cmp::min($size_bits, 8) {
                    if value & mask != ZERO {
                        buf[i] = U1_ONE;
                    }
                    mask >>= 1;
                }
                if $size_bits > 8 {
                    let mut mask = ONE << std::cmp::min($size_bits - 1, 15);
                    for i in 8..std::cmp::min($size_bits, 16) {
                        if value & mask != ZERO {
                            buf[i] = U1_ONE;
                        }
                        mask >>= 1;
                    }
                }
                if $size_bits > 16 {
                    let mut mask = ONE << std::cmp::min($size_bits - 1, 23);
                    for i in 16..std::cmp::min($size_bits, 24) {
                        if value & mask != ZERO {
                            buf[i] = U1_ONE;
                        }
                        mask >>= 1;
                    }
                }
                if $size_bits > 24 {
                    let mut mask = ONE << std::cmp::min($size_bits - 1, 31);
                    for i in 24..std::cmp::min($size_bits, 32) {
                        if value & mask != ZERO {
                            buf[i] = U1_ONE;
                        }
                        mask >>= 1;
                    }
                }
            }
        }
    };
}

macro_rules! impl_write_be {
    ($type:ty, $size_bits:expr) => {
        paste! {
            fn [<write_ $type>](buf: &mut [u1; $size_bits], mut value: $type) {
                const ONE: $type = init_integer!($type, 1);
                for i in (0..$size_bits).rev() {
                    if value & ONE == ONE {
                        buf[i] = U1_ONE;
                    }
                    value >>= 1;
                }
            }
        }
    };
}

pub trait ByteOrder {
    fn read_u9(buf: &[u1; 9]) -> u9;
    fn read_u10(buf: &[u1; 10]) -> u10;
    fn read_u11(buf: &[u1; 11]) -> u11;
    fn read_u12(buf: &[u1; 12]) -> u12;
    fn read_u13(buf: &[u1; 13]) -> u13;
    fn read_u14(buf: &[u1; 14]) -> u14;
    fn read_u15(buf: &[u1; 15]) -> u15;
    fn read_u16(buf: &[u1; 16]) -> u16;
    fn read_u17(buf: &[u1; 17]) -> u17;
    fn read_u18(buf: &[u1; 18]) -> u18;
    fn read_u19(buf: &[u1; 19]) -> u19;
    fn read_u20(buf: &[u1; 20]) -> u20;
    fn read_u21(buf: &[u1; 21]) -> u21;
    fn read_u22(buf: &[u1; 22]) -> u22;
    fn read_u23(buf: &[u1; 23]) -> u23;
    fn read_u24(buf: &[u1; 24]) -> u24;
    fn read_u25(buf: &[u1; 25]) -> u25;
    fn read_u26(buf: &[u1; 26]) -> u26;
    fn read_u27(buf: &[u1; 27]) -> u27;
    fn read_u28(buf: &[u1; 28]) -> u28;
    fn read_u29(buf: &[u1; 29]) -> u29;
    fn read_u30(buf: &[u1; 30]) -> u30;
    fn read_u31(buf: &[u1; 31]) -> u31;
    fn read_u32(buf: &[u1; 32]) -> u32;

    fn write_u9(buf: &mut [u1; 9], value: u9);
    fn write_u10(buf: &mut [u1; 10], value: u10);
    fn write_u11(buf: &mut [u1; 11], value: u11);
    fn write_u12(buf: &mut [u1; 12], value: u12);
    fn write_u13(buf: &mut [u1; 13], value: u13);
    fn write_u14(buf: &mut [u1; 14], value: u14);
    fn write_u15(buf: &mut [u1; 15], value: u15);
    fn write_u16(buf: &mut [u1; 16], value: u16);
    fn write_u17(buf: &mut [u1; 17], value: u17);
    fn write_u18(buf: &mut [u1; 18], value: u18);
    fn write_u19(buf: &mut [u1; 19], value: u19);
    fn write_u20(buf: &mut [u1; 20], value: u20);
    fn write_u21(buf: &mut [u1; 21], value: u21);
    fn write_u22(buf: &mut [u1; 22], value: u22);
    fn write_u23(buf: &mut [u1; 23], value: u23);
    fn write_u24(buf: &mut [u1; 24], value: u24);
    fn write_u25(buf: &mut [u1; 25], value: u25);
    fn write_u26(buf: &mut [u1; 26], value: u26);
    fn write_u27(buf: &mut [u1; 27], value: u27);
    fn write_u28(buf: &mut [u1; 28], value: u28);
    fn write_u29(buf: &mut [u1; 29], value: u29);
    fn write_u30(buf: &mut [u1; 30], value: u30);
    fn write_u31(buf: &mut [u1; 31], value: u31);
    fn write_u32(buf: &mut [u1; 32], value: u32);
}

pub struct BigEndian {}

pub struct LittleEndian {}

pub type NetworkOrder = BigEndian;

impl ByteOrder for LittleEndian {
    impl_read_le!(u9, 9);
    impl_read_le!(u10, 10);
    impl_read_le!(u11, 11);
    impl_read_le!(u12, 12);
    impl_read_le!(u13, 13);
    impl_read_le!(u14, 14);
    impl_read_le!(u15, 15);
    impl_read_le!(u16, 16);
    impl_read_le!(u17, 17);
    impl_read_le!(u18, 18);
    impl_read_le!(u19, 19);
    impl_read_le!(u20, 20);
    impl_read_le!(u21, 21);
    impl_read_le!(u22, 22);
    impl_read_le!(u23, 23);
    impl_read_le!(u24, 24);
    impl_read_le!(u25, 25);
    impl_read_le!(u26, 26);
    impl_read_le!(u27, 27);
    impl_read_le!(u28, 28);
    impl_read_le!(u29, 29);
    impl_read_le!(u30, 30);
    impl_read_le!(u31, 31);
    impl_read_le!(u32, 32);

    impl_write_le!(u9, 9);
    impl_write_le!(u10, 10);
    impl_write_le!(u11, 11);
    impl_write_le!(u12, 12);
    impl_write_le!(u13, 13);
    impl_write_le!(u14, 14);
    impl_write_le!(u15, 15);
    impl_write_le!(u16, 16);
    impl_write_le!(u17, 17);
    impl_write_le!(u18, 18);
    impl_write_le!(u19, 19);
    impl_write_le!(u20, 20);
    impl_write_le!(u21, 21);
    impl_write_le!(u22, 22);
    impl_write_le!(u23, 23);
    impl_write_le!(u24, 24);
    impl_write_le!(u25, 25);
    impl_write_le!(u26, 26);
    impl_write_le!(u27, 27);
    impl_write_le!(u28, 28);
    impl_write_le!(u29, 29);
    impl_write_le!(u30, 30);
    impl_write_le!(u31, 31);
    impl_write_le!(u32, 32);
}

impl ByteOrder for BigEndian {
    impl_read_be!(u9, 9);
    impl_read_be!(u10, 10);
    impl_read_be!(u11, 11);
    impl_read_be!(u12, 12);
    impl_read_be!(u13, 13);
    impl_read_be!(u14, 14);
    impl_read_be!(u15, 15);
    impl_read_be!(u16, 16);
    impl_read_be!(u17, 17);
    impl_read_be!(u18, 18);
    impl_read_be!(u19, 19);
    impl_read_be!(u20, 20);
    impl_read_be!(u21, 21);
    impl_read_be!(u22, 22);
    impl_read_be!(u23, 23);
    impl_read_be!(u24, 24);
    impl_read_be!(u25, 25);
    impl_read_be!(u26, 26);
    impl_read_be!(u27, 27);
    impl_read_be!(u28, 28);
    impl_read_be!(u29, 29);
    impl_read_be!(u30, 30);
    impl_read_be!(u31, 31);
    impl_read_be!(u32, 32);

    impl_write_be!(u9, 9);
    impl_write_be!(u10, 10);
    impl_write_be!(u11, 11);
    impl_write_be!(u12, 12);
    impl_write_be!(u13, 13);
    impl_write_be!(u14, 14);
    impl_write_be!(u15, 15);
    impl_write_be!(u16, 16);
    impl_write_be!(u17, 17);
    impl_write_be!(u18, 18);
    impl_write_be!(u19, 19);
    impl_write_be!(u20, 20);
    impl_write_be!(u21, 21);
    impl_write_be!(u22, 22);
    impl_write_be!(u23, 23);
    impl_write_be!(u24, 24);
    impl_write_be!(u25, 25);
    impl_write_be!(u26, 26);
    impl_write_be!(u27, 27);
    impl_write_be!(u28, 28);
    impl_write_be!(u29, 29);
    impl_write_be!(u30, 30);
    impl_write_be!(u31, 31);
    impl_write_be!(u32, 32);
}
