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
        n: usize, t: usize, a: usize,
    }

    let k = t + a;
    let u = n - k;

    if t > a {
        let a = a + u;
        if t < a {
            println!("No");
        } else {
            println!("Yes");
        }
    } else {
        let t = t + u;
        if a < t {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
