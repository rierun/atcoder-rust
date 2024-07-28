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
        mut h: usize
    }

    let mut day = 0;
    let mut s = 0;

    while s <= h {
        s += pow(2, day);
        day += 1;
        dbg!(s);
    }

    println!("{}", day);
}
