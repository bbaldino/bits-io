# bits-io

> **Flexible bit- and byte-level I/O operations for Rust.**

**bits-io** provides types and traits for handling both bits and bytes seamlessly.

While traditional Rust types like [`Bytes`](https://docs.rs/bytes) and
[`Buf`](https://docs.rs/bytes) (and their mutable counterparts) as well as the
standard library’s
[`Cursor`](https://doc.rust-lang.org/std/io/struct.Cursor.html),
[`Read`](https://doc.rust-lang.org/std/io/trait.Read.html), and
[`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) operate primarily
at the byte level, **bits-io** extends these capabilities by offering
fine-grained bit-level access alongside full support for familiar byte-oriented
workflows.

The types in **bits-io** are designed as supersets of their standard
counterparts:

- You can efficiently manipulate whole bytes, slices, and streams just as before.
- You can also access and manipulate individual bits when needed, without
  sacrificing performance or ergonomics.

---

## Main Types

| Type             | Description |
|------------------|-------------|
| **Bits**         | An immutable view over underlying data that supports bit-level operations alongside traditional byte-level access—akin to `Bytes`, but with bit-level APIs as well. |
| **BitsMut**      | A mutable, growable view that lets you work at both the byte and bit levels, similar in spirit to `BytesMut` with additional fine-grained control. |
| **BitBuf**       | A read-only buffer trait that matches `bytes::Buf` and adds bit-level operations. |
| **BitBufMut**    | A mutable buffer trait that matches `bytes::BufMut` and adds bit-level operations. |
| **BitCursor**    | A cursor that tracks the current position in a buffer by bit rather than by byte; `std::io::Cursor` for bits. |
| **BitRead**      | A trait analogous to `std::io::Read`, enabling both bite- and byte-level reads. |
| **BitWrite**     | A trait analagous `std::io::Write`, enabling both bit- and byte-level writes. |

---

## FAQ

### If this is just `bytes` with extra bit-level APIs, why not have `Buf` and `BufMut` be supertraits of `BitBuf` and `BitBufMut`?

The problem there is that then I wouldn't be able to implement `BitBuf` and
`BitBufMut` for bit-specific types I wanted to support like `BitSlice`.

But, `Bits` and `BitsMut` do implement `Buf` and `Buf` & `BufMut` respectively.
