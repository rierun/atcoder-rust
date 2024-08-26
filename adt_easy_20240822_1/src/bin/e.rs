#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
fn main() {
    input! {
       n: usize,
    }

    let mut f = vec![];

    for _i in 0..n {
        input! {
            s: Bytes,
        }
        let mut flag = true;
        if f[s] == 0 {
            println!("{}", s)
        }
    }
}
