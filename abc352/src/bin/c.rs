#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut h = 0;
    let mut hm = 0;

    for _ in 0..n {
        input! {
            a: usize, b: usize,
        }

        h += a;

        hm = max(hm, b - a)
    }

    println!("{}", h + hm);
}
