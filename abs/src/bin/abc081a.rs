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
        s: String,
    }
    let ans = s.chars().filter(|&c| c == '1').count();
    println!("{}", ans);
}
