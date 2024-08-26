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
        _n: usize, x: usize, y: usize, z: usize,
    }

    if x < z && z < y || y < z && z < x {
        println!("Yes");
    } else {
        println!("No");
    }
}
