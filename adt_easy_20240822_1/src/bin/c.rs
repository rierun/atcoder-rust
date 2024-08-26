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
        a: [usize; n],
    }
    // 0, 1.. nで aに含まれていないものをリストアップ

    let ans = 0..=n;

    for i in ans {
        let mut is_not_contain = true;
        for j in a.clone() {
            if i == j {
                is_not_contain = false;
                break;
            }
        }
        if is_not_contain {
            println!("{}", i);
            return;
        }
    }
}
