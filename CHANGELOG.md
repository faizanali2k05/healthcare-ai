# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0]

Initial release.

### Added

- `Monoid` trait describing an associative operation with an identity element.
- `SegmentTree<M>` — iterative segment tree generic over any `Monoid`, with
  `new`, `from_slice`, `len`, `is_empty`, `get`, `update`, and `query`.
- `FenwickTree<T>` — binary indexed tree over an additive type, with `new`,
  `from_slice`, `len`, `is_empty`, `add`, `prefix_sum`, and `range_sum`.
- `Sum<T>` — ready-made sum monoid for any `Clone + Default + Add` type.
- `no_std` support via the default-on `std` feature (disable for `alloc`-only
  targets).

[0.1.0]: https://github.com/faizanali2k05/segfen/releases/tag/v0.1.0
