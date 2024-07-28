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
        a: usize, b: usize,
    }
    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" });
}
