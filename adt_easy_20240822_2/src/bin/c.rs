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
       _n: usize,
       t: Chars,
    }

    let mut heading = 0;

    let mut x = 0;
    let mut y = 0;

    for i in t {
        if i == 'S' {
            if heading == 0 {
                x += 1;
            } else if heading == 1 {
                y -= 1;
            } else if heading == 2 {
                x -= 1;
            } else {
                y += 1;
            }
        } else {
            if heading == 3 {
                heading = 0;
            } else {
                heading += 1;
            }
        }
    }

    println!("{} {}", x, y);
}
