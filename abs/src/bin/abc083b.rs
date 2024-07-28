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
        n: usize, a: usize, b: usize,
    }
    let mut ans = 0;
    for i in 0..=n {
        let sum = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum::<usize>();
        if a <= sum && sum <= b {
            ans += i;
        }
    }

    println!("{}", ans);
}
