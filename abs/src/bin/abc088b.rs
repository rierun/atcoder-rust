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
        mut a: [usize; n],
    }

    a.sort_unstable_by(|a, b| b.cmp(a));
    let mut alice = 0;
    let mut bob = 0;
    for (i, &x) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += x;
        } else {
            bob += x;
        }
    }

    println!("{}", alice - bob);
}
