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
        n:usize,
        a: [usize; n]
    }

    let mut b = a.clone();
    b.sort();
    let tar = b[n - 2];

    dbg!(a.clone());

    let mut ans = 0;
    for i in 0..n {
        ans += 1;
        if a[i] == tar {
            println!("{}", ans);
            return;
        }
    }
}
