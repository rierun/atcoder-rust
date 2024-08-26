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
        n: usize, k: usize,
        mut a: [usize; n],
    }

    let mut cnt = 0;
    let mut b = k;
    for i in a {
        if i > b {
            cnt += 1;
            b = k - i;
        } else {
            b -= i;
        }
    }

    println!("{}", cnt + 1);
}
