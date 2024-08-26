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
       mut a: [usize; n],
    }

    let mut b = Vec::new();

    for i in 0..n {
        b.push(a[i]);

        while b.len() >= 2 && b[b.len() - 1] == b[b.len() - 2] {
            let x = b[b.len() - 1];
            b.pop();
            b.pop();
            b.push(x + 1);
        }
    }

    println!("{}", b.len());
}
