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
        y: usize
    }

    if y % 4 == 0 {
        if y % 100 != 0 {
            println!("366");
        } else if y % 400 == 0 {
            println!("366");
        } else {
            println!("365");
        }
    } else {
        println!("365");
    }
}
