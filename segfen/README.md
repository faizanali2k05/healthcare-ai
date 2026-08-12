# segfen

Segment tree (generic over any user-defined monoid: sum, min, max, gcd, or, matrix product…) and Fenwick / binary indexed tree. `O(log n)` point update + range query. `no_std` + `alloc` compatible (disable the default `std` feature).

```rust
use segfen::{SegmentTree, Monoid, FenwickTree};

struct Min;
impl Monoid for Min {
    type T = i64;
    fn identity() -> i64 { i64::MAX }
    fn combine(a: &i64, b: &i64) -> i64 { *a.min(b) }
}

let st = SegmentTree::<Min>::from_slice(&[5, 2, 8, 1, 9]);
assert_eq!(st.query(0..5), 1);
```

License: MIT OR Apache-2.0
