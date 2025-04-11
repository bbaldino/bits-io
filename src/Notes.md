`std::io::Cursor`:

```rust
struct Cursor<T> {
  inner: T,
  pos: u64 // 'byte' level position
}
```

`BitCursor`:

```rust
struct BitCursor<T> {
  inner: T,
  pos: u64 // 'bit' level position
}
```

The APIs will be exactly the same, just with `BitCursor`'s applying at a bit-level where it makes sense:

`get_mut`, `get_ref`, `into_inner`, `new` all work exactly the same

`position` and `set_position` will use bit-indices as opposed to byte for `BitCursor`

`split` and `split_mut` will return `(&BitSlice<u8, Msb0>, &BitSlice<u8, Msb0>)` for `BitCursor` instead of `(&[u8], &[u8])`.

Implementing split/split_mut:

At first I did blanket implementations of `BitCursor<T>` for any T that was
`AsRef<BitSlice<u8, Msb0>>`.  For the most part this worked well, but it meant
that `BitCursor<&[u8]>` no longer worked since it didn't meet that requirement.
Then I discovered `bitvec` defines an `AsBits` trait that it claims is the
equivalent for `AsRef`, so I tried that instead, but that results in another
problem: for some reason `BitVec`` doesn't implement`AsBits`, it just provides
its own`as_bitslice` method.  From what I can tell, this means there's no
single blanket trait boundary I can use to provide an implementation that works
for everything.

For now I think I'll stick with AsRef, but hopefully can figure something
better out long term.
--> Actually, it looks like there isn't an impl for AsRef<BitSlice<..>> for
Vec<u8>, so maybe AsBits is better for now and I can manually do some
implementations for BitCursor<BitVec<..>>? Will give that a try.
--> **Actually** this is flawed: it looks like AsRef _is_ what I want, and, in
order to make things like Vec<u8> work I'd just need to do some extra stuff
with a helper trait.  But, maybe this makes sense since Vec<u8> and &[u8] are
explicitly byte-level abstractions and need to be 'adapted'?

----> The way forward here ended up being adding a separate helper trait that
can bridge all the gaps and using that
