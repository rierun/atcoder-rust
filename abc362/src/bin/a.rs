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
        r: usize, g: usize, b: usize,
        c: String,
    }

    if c == "Blue" {
        println!("{}", min(r, g))
    } else if c == "Red" {
        println!("{}", min(g, b))
    } else {
        println!("{}", min(b, r))
    }
}
