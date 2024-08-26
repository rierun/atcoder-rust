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
       a: [Chars; n],
       b: [Chars; n],
    }

    for x in 0..n {
        for y in 0..n {
            if a[x][y] != b[x][y] {
                println!("{} {}", x + 1, y + 1);
                return;
            }
        }
    }
}
