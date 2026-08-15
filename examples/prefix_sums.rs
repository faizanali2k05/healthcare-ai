//! Running balances with a Fenwick tree, plus the ready-made `Sum` monoid.
//!
//! Run with: `cargo run --example prefix_sums`

use segfen::{FenwickTree, SegmentTree, Sum};

fn main() {
    // Daily cash-flow deltas.
    let deltas = [100i64, -30, 45, -60, 25, 10, -15];

    let mut fw = FenwickTree::from_slice(&deltas);
    println!("deltas: {deltas:?}\n");

    for day in 1..=deltas.len() {
        println!("balance after day {day}: {}", fw.prefix_sum(day));
    }

    println!("\nsum of days 2..5: {}", fw.range_sum(2..5));

    // A correction to day 0 propagates to every later prefix in O(log n).
    fw.add(0, 500);
    println!(
        "after add(0, 500), final balance: {}",
        fw.prefix_sum(deltas.len())
    );

    // The same sums via a segment tree using the built-in `Sum` monoid — this
    // one supports assignment instead of addition.
    let mut st = SegmentTree::<Sum<i64>>::from_slice(&deltas);
    println!("\nsegment tree sum of 2..5: {}", st.query(2..5));
    st.update(2, 0);
    println!("after update(2, 0):       {}", st.query(2..5));
}
