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

    let mut e = k;
    let mut t = 0;

    for i in a {
        if e < i {
            t += 1;
            e = k;
        }
        e -= i;
    }

    println!("{}", t + 1);
}
