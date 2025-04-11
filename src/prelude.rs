// Re-export bitvec core types so users don't need to depend on bitvec directly
// pub use bitvec::{access::BitSafeU8, order::Msb0, slice::BitSlice as OgBitSlice, store::BitStore};
pub use bitvec::{access::BitSafeU8, order::Msb0};

// Re-export your safe aliases (always Msb0 + u8-based)
pub use crate::bit_types::{BitSlice, BitSliceMut, BitStore, BitVec};

// Core traits
pub use crate::bit_cursor::BitCursor;
pub use crate::bit_read::BitRead;
pub use crate::bit_write::BitWrite;
pub use crate::byte_order::{BigEndian, ByteOrder, LittleEndian};
