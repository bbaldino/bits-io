# BitCursor

BitCursor is similar to std::io::Cursor, but allows reading various amounts of bits from a given buffer in addition
to byte-sized chunks.

### Examples
```rust
let data: Vec<u8> = vec![0b11110000, 0b00001111];
let mut cursor = BitCursor::new(data);

assert_eq!(cursor.read_u4().unwrap(), 15);
assert_eq!(cursor.read_u4().unwrap(), 0);
assert_eq!(cursor.read_u2().unwrap(), 0);
assert_eq!(cursor.read_u6().unwrap(), 15);
```
