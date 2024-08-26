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
       mut s: usize,mut  t: usize,mut x: usize,
    }

    if x < t {
        if s <= x && x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if s <= x || x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
