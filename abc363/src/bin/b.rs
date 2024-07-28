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
        n: usize, t: usize, p: usize,
        mut l: [usize; n],
    }

    let mut cnt = 0;
    let mut days = 0;

    for i in &l {
        if *i >= t {
            cnt += 1;
        }
    }

    while cnt < p {
        days += 1;

        cnt = 0;

        for i in 0..l.len() {
            l[i] += 1;
            if l[i] >= t {
                cnt += 1;
            }
        }

        dbg!(days, cnt);
        dbg!(&l);
    }

    println!("{}", days);
}
