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
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut cnt = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
