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
        s: Chars,
    }

    let num = s[3..6].iter().collect::<String>().parse::<i32>().unwrap();

    if 0 < num && num < 350 && num != 316 {
        println!("Yes");
    } else {
        println!("No");
    }
}
