use std::cmp::{max, min};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64, b: u64, c: u64, d: u64, e: u64, f: u64,
        g: u64, h: u64, i: u64, j: u64, k: u64, l: u64,
    }

    fn kf(l1: u64, r1: u64, l2: u64, r2: u64) -> bool {
        return max(l1, l2) < min(r1, r2);
    }

    if kf(a, d, g, j) && kf(b, e, h, k) && kf(c, f, i, l) {
        println!("Yes")
    } else {
        println!("No")
    }
}
