//! # segfen
//!
//! Range-query data structures for Rust: a [`SegmentTree`] generic over any
//! user-defined [`Monoid`], and a [`FenwickTree`] (binary indexed tree) for
//! prefix sums. Both give `O(log n)` point updates and range queries.
//!
//! Unlike ad-hoc implementations scattered across crates.io, the segment tree
//! here is *generic over the operation*: sum, min, max, gcd, bitwise-or,
//! matrix product — anything associative with an identity.
//!
//! ```
//! use segfen::{SegmentTree, Monoid, FenwickTree};
//!
//! // Range-minimum query via a custom monoid.
//! struct Min;
//! impl Monoid for Min {
//!     type T = i64;
//!     fn identity() -> i64 { i64::MAX }
//!     fn combine(a: &i64, b: &i64) -> i64 { *a.min(b) }
//! }
//!
//! let mut st = SegmentTree::<Min>::from_slice(&[5, 2, 8, 1, 9]);
//! assert_eq!(st.query(0..5), 1);
//! assert_eq!(st.query(0..3), 2);
//! st.update(3, 100);
//! assert_eq!(st.query(2..5), 8);
//!
//! // Prefix sums via a Fenwick tree.
//! let mut fw = FenwickTree::from_slice(&[1i64, 2, 3, 4]);
//! assert_eq!(fw.range_sum(1..3), 5);
//! fw.add(0, 10);
//! assert_eq!(fw.prefix_sum(1), 11);
//! ```
//!
//! `no_std` + `alloc` compatible: disable the default `std` feature.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::vec::Vec;
use core::ops::{AddAssign, Range, Sub};

/// An associative operation with an identity element.
///
/// Laws the implementation must uphold:
/// - `combine(identity(), x) == x == combine(x, identity())`
/// - `combine(combine(a, b), c) == combine(a, combine(b, c))`
pub trait Monoid {
    /// Element type stored in the tree.
    type T: Clone;
    /// The identity element.
    fn identity() -> Self::T;
    /// The associative binary operation.
    fn combine(a: &Self::T, b: &Self::T) -> Self::T;
}

/// Ready-made sum monoid for any numeric-like type.
pub struct Sum<T>(core::marker::PhantomData<T>);

impl<T> Monoid for Sum<T>
where
    T: Clone + Default + core::ops::Add<Output = T>,
{
    type T = T;
    fn identity() -> T {
        T::default()
    }
    fn combine(a: &T, b: &T) -> T {
        a.clone() + b.clone()
    }
}

/// An iterative segment tree supporting point update and range query in
/// `O(log n)`, generic over a [`Monoid`].
#[derive(Clone, Debug)]
pub struct SegmentTree<M: Monoid> {
    n: usize,
    tree: Vec<M::T>,
}

impl<M: Monoid> SegmentTree<M> {
    /// Builds a tree of `n` identity elements.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            tree: alloc::vec![M::identity(); 2 * n.max(1)],
        }
    }

    /// Builds a tree from a slice in `O(n)`.
    #[must_use]
    pub fn from_slice(values: &[M::T]) -> Self {
        let n = values.len();
        let mut tree = alloc::vec![M::identity(); 2 * n.max(1)];
        tree[n..n + n].clone_from_slice(values);
        for i in (1..n).rev() {
            tree[i] = M::combine(&tree[2 * i], &tree[2 * i + 1]);
        }
        Self { n, tree }
    }

    /// Number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns `true` if the tree holds no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the element at `index`.
    ///
    /// # Panics
    /// Panics if `index >= len()`.
    #[must_use]
    pub fn get(&self, index: usize) -> &M::T {
        assert!(index < self.n, "index {index} out of bounds ({})", self.n);
        &self.tree[self.n + index]
    }

    /// Sets the element at `index` to `value` in `O(log n)`.
    ///
    /// # Panics
    /// Panics if `index >= len()`.
    pub fn update(&mut self, index: usize, value: M::T) {
        assert!(index < self.n, "index {index} out of bounds ({})", self.n);
        let mut i = self.n + index;
        self.tree[i] = value;
        while i > 1 {
            i /= 2;
            self.tree[i] = M::combine(&self.tree[2 * i], &self.tree[2 * i + 1]);
        }
    }

    /// Combines all elements in `range` in `O(log n)`. An empty range yields
    /// the identity.
    ///
    /// # Panics
    /// Panics if `range.end > len()` or `range.start > range.end`.
    #[must_use]
    pub fn query(&self, range: Range<usize>) -> M::T {
        assert!(range.start <= range.end && range.end <= self.n, "bad range");
        let (mut l, mut r) = (self.n + range.start, self.n + range.end);
        let (mut left, mut right) = (M::identity(), M::identity());
        while l < r {
            if l % 2 == 1 {
                left = M::combine(&left, &self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = M::combine(&self.tree[r], &right);
            }
            l /= 2;
            r /= 2;
        }
        M::combine(&left, &right)
    }
}

/// A Fenwick tree (binary indexed tree) for `O(log n)` prefix sums and point
/// additions over an additive type.
#[derive(Clone, Debug)]
pub struct FenwickTree<T> {
    tree: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Clone + Default + AddAssign + Sub<Output = T>,
{
    /// Builds a tree of `n` zeros (`T::default()`).
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            tree: alloc::vec![T::default(); n + 1],
        }
    }

    /// Builds a tree from a slice in `O(n)`.
    #[must_use]
    pub fn from_slice(values: &[T]) -> Self {
        let n = values.len();
        let mut tree = alloc::vec![T::default(); n + 1];
        for (i, v) in values.iter().enumerate() {
            let i = i + 1;
            tree[i] += v.clone();
            let parent = i + (i & i.wrapping_neg());
            if parent <= n {
                let add = tree[i].clone();
                tree[parent] += add;
            }
        }
        Self { tree }
    }

    /// Number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }

    /// Returns `true` if the tree holds no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds `delta` to the element at `index` in `O(log n)`.
    ///
    /// # Panics
    /// Panics if `index >= len()`.
    pub fn add(&mut self, index: usize, delta: T) {
        assert!(index < self.len(), "index out of bounds");
        let mut i = index + 1;
        while i < self.tree.len() {
            self.tree[i] += delta.clone();
            i += i & i.wrapping_neg();
        }
    }

    /// Sum of elements in `0..end` in `O(log n)`.
    ///
    /// # Panics
    /// Panics if `end > len()`.
    #[must_use]
    pub fn prefix_sum(&self, end: usize) -> T {
        assert!(end <= self.len(), "end out of bounds");
        let mut acc = T::default();
        let mut i = end;
        while i > 0 {
            acc += self.tree[i].clone();
            i -= i & i.wrapping_neg();
        }
        acc
    }

    /// Sum of elements in `range` in `O(log n)`.
    ///
    /// # Panics
    /// Panics if `range.end > len()` or `range.start > range.end`.
    #[must_use]
    pub fn range_sum(&self, range: Range<usize>) -> T {
        assert!(range.start <= range.end, "bad range");
        self.prefix_sum(range.end) - self.prefix_sum(range.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Max;
    impl Monoid for Max {
        type T = i32;
        fn identity() -> i32 {
            i32::MIN
        }
        fn combine(a: &i32, b: &i32) -> i32 {
            *a.max(b)
        }
    }

    struct Gcd;
    impl Monoid for Gcd {
        type T = u64;
        fn identity() -> u64 {
            0
        }
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

    #[test]
    fn segment_tree_sum_matches_naive() {
        let data: Vec<i64> = (0..97).map(|i| (i * 37 % 101) - 50).collect();
        let st = SegmentTree::<Sum<i64>>::from_slice(&data);
        for l in 0..=data.len() {
            for r in l..=data.len() {
                let naive: i64 = data[l..r].iter().sum();
                assert_eq!(st.query(l..r), naive, "range {l}..{r}");
            }
        }
    }

    #[test]
    fn segment_tree_max_with_updates() {
        let mut st = SegmentTree::<Max>::from_slice(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(st.query(0..8), 9);
        assert_eq!(st.query(0..5), 5);
        st.update(5, -1);
        assert_eq!(st.query(0..8), 6);
        assert_eq!(*st.get(5), -1);
    }

    #[test]
    fn segment_tree_gcd() {
        let st = SegmentTree::<Gcd>::from_slice(&[12, 18, 24, 5]);
        assert_eq!(st.query(0..3), 6);
        assert_eq!(st.query(0..4), 1);
        assert_eq!(st.query(2..2), 0); // empty -> identity
    }

    #[test]
    fn segment_tree_empty_and_single() {
        let st = SegmentTree::<Sum<i32>>::new(0);
        assert!(st.is_empty());
        assert_eq!(st.query(0..0), 0);
        let st = SegmentTree::<Sum<i32>>::from_slice(&[42]);
        assert_eq!(st.query(0..1), 42);
    }

    #[test]
    fn fenwick_matches_naive() {
        let data: Vec<i64> = (0..64).map(|i| (i * 13 % 29) - 14).collect();
        let fw = FenwickTree::from_slice(&data);
        for l in 0..=data.len() {
            for r in l..=data.len() {
                let naive: i64 = data[l..r].iter().sum();
                assert_eq!(fw.range_sum(l..r), naive, "range {l}..{r}");
            }
        }
    }

    #[test]
    fn fenwick_add() {
        let mut fw = FenwickTree::new(5);
        fw.add(0, 1i64);
        fw.add(4, 10);
        fw.add(2, -3);
        assert_eq!(fw.prefix_sum(5), 8);
        assert_eq!(fw.range_sum(1..5), 7);
        assert_eq!(fw.range_sum(3..3), 0);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_panics() {
        let st = SegmentTree::<Sum<i32>>::from_slice(&[1, 2]);
        let _ = st.query(0..3);
    }
}
