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
        q: usize,
    }
    let mut v: Vec<usize> = vec![0; 1000001];
    let mut a = 0;

    for _ in 0..q {
        input! {
            s: usize,
        }
        if s == 1 {
            input! {
                x: usize,
            }
            v[x] = v[x] + 1;
            if v[x] == 1 {
                a = a + 1;
            }
        } else if s == 2 {
            input! {
                x: usize,
            }
            v[x] = v[x] - 1;
            if v[x] == 0 {
                a = a - 1;
            }
        } else {
            println!("{}", a);
        }
    }
}
