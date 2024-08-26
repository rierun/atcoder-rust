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
        t: Chars,
    }

    let mut cnt = 0;

    let mut b = Vec::new();

    for i in 0..t.len() {
        if t[i] == s[cnt] {
            cnt += 1;
            b.push(i + 1);
        }
    }

    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
