# segfen

[![crates.io](https://img.shields.io/crates/v/segfen.svg)](https://crates.io/crates/segfen)
[![docs.rs](https://docs.rs/segfen/badge.svg)](https://docs.rs/segfen)
[![license](https://img.shields.io/crates/l/segfen.svg)](#license)

Range-query data structures for Rust:

- **`SegmentTree<M>`** — an iterative segment tree generic over any user-defined
  [monoid](https://en.wikipedia.org/wiki/Monoid): sum, min, max, gcd, bitwise-or,
  matrix product — anything associative with an identity.
- **`FenwickTree<T>`** — a Fenwick tree (binary indexed tree) for prefix sums and
  point additions over an additive type.

Both provide `O(log n)` point updates and range queries, with `O(n)` construction
from a slice. No dependencies, no `unsafe`, `no_std` compatible.

## Installation

```toml
[dependencies]
segfen = "0.1"
```

Or:

```sh
cargo add segfen
```

## Quick start

```rust
use segfen::{FenwickTree, Monoid, SegmentTree};

// Range-minimum query via a custom monoid.
struct Min;
impl Monoid for Min {
    type T = i64;
    fn identity() -> i64 { i64::MAX }
    fn combine(a: &i64, b: &i64) -> i64 { *a.min(b) }
}

let mut st = SegmentTree::<Min>::from_slice(&[5, 2, 8, 1, 9]);
assert_eq!(st.query(0..5), 1);
assert_eq!(st.query(0..3), 2);

st.update(3, 100);
assert_eq!(st.query(2..5), 8);

// Prefix sums via a Fenwick tree.
let mut fw = FenwickTree::from_slice(&[1i64, 2, 3, 4]);
assert_eq!(fw.range_sum(1..3), 5);
fw.add(0, 10);
assert_eq!(fw.prefix_sum(1), 11);
```

## Defining a monoid

Implement the [`Monoid`] trait for a marker type. The implementation must uphold
two laws:

- **Identity** — `combine(identity(), x) == x == combine(x, identity())`
- **Associativity** — `combine(combine(a, b), c) == combine(a, combine(b, c))`

```rust
use segfen::{Monoid, SegmentTree};

struct Gcd;
impl Monoid for Gcd {
    type T = u64;
    fn identity() -> u64 { 0 } // gcd(0, x) == x
    fn combine(a: &u64, b: &u64) -> u64 {
        let (mut a, mut b) = (*a, *b);
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
}

let st = SegmentTree::<Gcd>::from_slice(&[12, 18, 24, 5]);
assert_eq!(st.query(0..3), 6);
assert_eq!(st.query(0..4), 1);
```

A ready-made [`Sum`] monoid is provided for any type implementing
`Clone + Default + Add`:

```rust
use segfen::{Sum, SegmentTree};

let st = SegmentTree::<Sum<i64>>::from_slice(&[1, 2, 3, 4, 5]);
assert_eq!(st.query(1..4), 9);
```

## Choosing between the two

| | `SegmentTree<M>` | `FenwickTree<T>` |
|---|---|---|
| Operation | any monoid | addition only |
| Query | arbitrary range | prefix / range sum |
| Update | assign (`update`) | add delta (`add`) |
| Memory | `2n` elements | `n + 1` elements |
| Requires | `Monoid` impl | `Clone + Default + AddAssign + Sub` |

Use `FenwickTree` when you only need sums — it is smaller and has a tighter
constant factor. Use `SegmentTree` for anything else.

## Complexity

| Operation | `SegmentTree` | `FenwickTree` |
|---|---|---|
| `from_slice` | `O(n)` | `O(n)` |
| `new` | `O(n)` | `O(n)` |
| `query` / `range_sum` | `O(log n)` | `O(log n)` |
| `update` / `add` | `O(log n)` | `O(log n)` |
| `get` | `O(1)` | — |

## `no_std`

The crate is `no_std` compatible and requires only `alloc`. Disable the default
`std` feature:

```toml
[dependencies]
segfen = { version = "0.1", default-features = false }
```

## Panics

Index and range arguments are checked. Methods panic on out-of-bounds access —
see the per-method documentation on [docs.rs](https://docs.rs/segfen). Empty
ranges are valid and yield the monoid identity.

## Minimum supported Rust version

Rust **1.75**. Raising the MSRV is considered a breaking change and will only
happen in a minor version bump while `0.x`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[`Monoid`]: https://docs.rs/segfen/latest/segfen/trait.Monoid.html
[`Sum`]: https://docs.rs/segfen/latest/segfen/struct.Sum.html
