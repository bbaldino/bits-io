# BitCursor

BitCursor is similar to std::io::Cursor, but allows reading various amounts of bits from a given buffer in addition
to byte-sized chunks.  It's built on top of the [ux](https://crates.io/crates/ux) crate for types.


# Design

## Traits

### `BitRead`
`BitRead` is analogus to the [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait, but its API is defined in terms of reading from "bit slices" instead of `u8` slices (`&[u8]`) like `std::io::Read`.


### `BitWrite`
`BitWrite` is analogus to the [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) trait, but its API is defined in terms of writing to "bit slices" instead of `u8` slices (`&[u8]`) like `std::io::Write`.


## Types

### `BitCursor`
`BitCursor` is analgous to the [`std::io::Cursor`](https://doc.rust-lang.org/std/io/struct.Cursor.html) type, but its API is defined in terms of bits instead of bytes.

### `BitSlice`/`BitSliceMut`
The `std::io` types' APIs often use the `&[u8]` type for 'slices', so `BitCursor`'s equivalent would be to use a slice of `u1`: `&[u1]`, but, for ergonomic reasons, we instead use `BitSlice`/`BitSliceMut` types to mimic `&[u1]` and `&mut [u1]`.
