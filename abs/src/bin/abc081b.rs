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

    let mut ans = 0;
    loop {
        if a.iter().all(|&x| x % 2 == 0) {
            a = a.iter().map(|&x| x / 2).collect();
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
