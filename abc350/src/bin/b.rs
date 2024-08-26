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
       n: usize, q: usize,
       t: [usize; q],
    }

    let mut b = vec![true; n];

    for i in t {
        b[i - 1] = !b[i - 1];
    }

    let mut cnt = 0;
    for i in b {
        if i {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
