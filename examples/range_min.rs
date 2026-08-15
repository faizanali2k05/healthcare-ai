//! Range-minimum queries over a custom monoid, with point updates.
//!
//! Run with: `cargo run --example range_min`

use segfen::{Monoid, SegmentTree};

/// Minimum monoid: `identity` is the largest possible value, so combining with
/// it never changes the result.
struct Min;

impl Monoid for Min {
    type T = i64;

    fn identity() -> i64 {
        i64::MAX
    }

    fn combine(a: &i64, b: &i64) -> i64 {
        *a.min(b)
    }
}

fn main() {
    let temperatures = [12, 7, 19, 3, 15, 8, 21, 5];
    let mut st = SegmentTree::<Min>::from_slice(&temperatures);

    println!("data: {temperatures:?}");
    println!("coldest overall:      {}", st.query(0..temperatures.len()));
    println!("coldest in 0..4:      {}", st.query(0..4));
    println!("coldest in 4..8:      {}", st.query(4..8));
    println!("empty range identity: {}", st.query(2..2));

    // Point update: the reading at index 3 was wrong.
    st.update(3, 30);
    println!("\nafter update(3, 30):");
    println!("coldest overall:      {}", st.query(0..temperatures.len()));
    println!("value at index 3:     {}", st.get(3));
}
