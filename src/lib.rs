#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod internal {
    pub mod bitvec {
        pub use bitvec::*;
    }
}
pub mod bit_types;
pub mod buf;
pub mod io;
pub mod prelude;

pub use nsw_types;
