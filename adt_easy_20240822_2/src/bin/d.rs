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
       h: usize, w: usize,
       c: [Chars; h]
    }

    let mut cnt = vec![0; w];

    for i in c {
        for j in 0..w {
            if i[j] == '#' {
                cnt[j] += 1;
            }
        }
    }

    println!(
        "{}",
        cnt.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
