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
        mut h: [isize; n],
    }

    let mut t = 0;

    for mut i in h {
        let num = i / 5;
        t += num * 3;
        i -= num * 5;
        while i > 0 {
            t += 1;
            if t % 3 == 0 {
                i -= 3;
            } else {
                i -= 1;
            }
        }
    }

    println!("{}", t);
}
