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
    let mut cnt = n;

    while cnt >= 2 {
        ans += 1;
        // 降順ソート
        a.sort_by(|a, b| b.cmp(a));
        dbg!(a.clone());
        for i in 0..=1 {
            if a[i] != 0 {
                a[i] -= 1;
                if a[i] == 0 {
                    cnt -= 1;
                }
            }
        }
    }

    println!("{}", ans);
}
