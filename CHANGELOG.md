# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.1](https://github.com/bbaldino/bits-io/compare/v0.6.0...v0.6.1) - 2025-05-02

### Fixed

- fix get_u8

## [0.6.0](https://github.com/bbaldino/bits-io/compare/v0.5.6...v0.6.0) - 2025-05-02

### Fixed

- optimize reading byte-aligned types from a byte-aligned buffer ([#23](https://github.com/bbaldino/bits-io/pull/23))

## [0.5.6](https://github.com/bbaldino/bits-io/compare/v0.5.5...v0.5.6) - 2025-05-01

### Added

- re-export BitSliceUxExts

### Fixed

- fix partialeq impl for Bits

## [0.5.5](https://github.com/bbaldino/bits-io/compare/v0.5.4...v0.5.5) - 2025-04-30

### Added

- add Bits::copy_from_bytes constructor

## [0.5.4](https://github.com/bbaldino/bits-io/compare/v0.5.3...v0.5.4) - 2025-04-29

### Fixed

- upgrade nsw-types version

## [0.5.3](https://github.com/bbaldino/bits-io/compare/v0.5.2...v0.5.3) - 2025-04-29

### Fixed

- fix some BitBuf impl code for BitCursor<&BitSlice>, add BitBuf impl
- properly limit slice returned by slice_bytes from a Bits instance

### Other

- tweak readme

## [0.5.2](https://github.com/bbaldino/bits-io/compare/v0.5.1...v0.5.2) - 2025-04-27

### Other

- link README
- update README

## [0.5.1](https://github.com/bbaldino/bits-io/compare/v0.5.0...v0.5.1) - 2025-04-26

### Added

- add 'limit' api to BitBufMut
- add 'chain' for BitBuf and BitBufMut, add more BitBufMut impls

## [0.5.0](https://github.com/bbaldino/bits-io/compare/v0.4.0...v0.5.0) - 2025-04-26

### Other

- rename all 'bits' methods to include bits in the names to make things more obvious

## [0.4.0](https://github.com/bbaldino/bits-io/compare/v0.3.0...v0.4.0) - 2025-04-24

### Other

- update nsw-types/remove unneeded num-traits feature
- fix impl of remaining for BitBufMut
- rename BitTake -> Take
- fix up some unit tests

## [0.3.0](https://github.com/bbaldino/bits-io/compare/v0.2.0...v0.3.0) - 2025-04-23

### Added

- add support for 'take' in BitBuf
- add try version of copy_to_slice
- experimenting with some byte-level operations for BitBuf.  If they work out can add matching ones to BitBufMut
- new constructors for Bits

### Fixed

- add advance_bytes to BitBuf
- use try_ versions of copy slice methods for get/put_uXX
- fix an issue where BitsMut didn't increase capacity when chunk_mut_bytes was called

### Other

- add unit test for BitBuf::byte_aligned
- add BitBuf impls for some basic types
- add chunk_mut_bytes, try_put_slice, try_put_slice_bytes, byte_aligned_mut methods to BitBufMut
- add try_copy_to_slice_bytes and byte_aligned methods to BitBuf

## [0.2.0](https://github.com/bbaldino/bits-io/compare/v0.1.7...v0.2.0) - 2025-04-21

### Other

- get rid of bit_read_exts/bit_write_exts.  these methods exist now on buf/bufmut
- Bit buf mut, lots of other fixes/cleanup ([#13](https://github.com/bbaldino/bits-io/pull/13))
- reorganize code, bring in bits/bitsmut/buf/bufmut ([#11](https://github.com/bbaldino/bits-io/pull/11))
- readme tweaks

## [0.1.7](https://github.com/bbaldino/bits-io/compare/v0.1.6...v0.1.7) - 2025-04-12

### Fixed

- expose u8, msb0 versions of some bitvec macros. clean up some code

### Other

- add some basic docs to BitRead and BitWrite

## [0.1.6](https://github.com/bbaldino/bitcursor/compare/v0.1.5...v0.1.6) - 2025-04-11

### Other

- README tweaks.  needs more work

## [0.1.5](https://github.com/bbaldino/bitcursor/compare/v0.1.4...v0.1.5) - 2025-03-25

### Added

- change subcursor to trait & support all range types

## [0.1.4](https://github.com/bbaldino/bitcursor/compare/v0.1.3...v0.1.4) - 2025-03-14

### Added

- update nsw-types

## [0.1.3](https://github.com/bbaldino/bitcursor/compare/v0.1.2...v0.1.3) - 2025-03-06

### Other

- update nsw-types version
- Update README.md

## [0.1.2](https://github.com/bbaldino/bitcursor/compare/v0.1.1...v0.1.2) - 2024-09-06

### Fixed
- use num_traits from nsw_types
