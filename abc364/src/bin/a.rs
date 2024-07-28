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
        s: [String; n],
    }
    let mut cnt = 0;
    let mut now = 0;
    for i in s {
        now += 1;
        if i == "sweet" {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt == 2 && now != n {
            dbg!("ok");
            println!("No");
            return;
        }
    }
    println!("Yes");
}
