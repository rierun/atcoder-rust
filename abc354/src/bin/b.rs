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
        mut p: [(String, usize); n],
    }

    let mut sum = 0;
    for i in 0..n {
        sum += p[i].1;
    }

    p.sort_by(|a, b| a.0.cmp(&b.0));

    let m = sum % n;

    let ans = p.iter().enumerate().nth(m).unwrap().1.clone().0;

    println!("{}", ans)
}
