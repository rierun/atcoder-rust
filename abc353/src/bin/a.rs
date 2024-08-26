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
        h: [usize; n],
    }

    for i in 1..n {
        if h[0] < h[i] {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
