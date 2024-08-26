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
        mut a: usize, mut b: usize,  mut c: usize,
    }

    let mut waked = false;
    let mut ans = false;
    for i in 0..48 {
        if i == 24 {
            a += 24;
            b += 24;
            c += 24;
        }

        if i == b {
            waked = true;
        }

        if i == a {
            if waked {
                ans = true;
                break;
            } else {
                ans = false;
            }
        }

        if i == c {
            waked = false;
        }
    }

    if ans {
        println!("No");
    } else {
        println!("Yes");
    }
}
