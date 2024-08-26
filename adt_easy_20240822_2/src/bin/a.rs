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
        s: Chars,
    }

    let mut ans = true;
    if s[0].is_lowercase() {
        ans = false;
    }

    for i in 1..s.len() {
        if s[i].is_uppercase() {
            ans = false;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
