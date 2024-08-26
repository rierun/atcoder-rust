use itertools::Itertools;
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};

fn s(i: usize, a: Vec<usize>, n: usize, k: usize, r: Vec<usize>) {
    if i == n {
        if a.iter().sum::<usize>() % k == 0 {
            println!("{}", a.iter().join(" "));
        }
    } else {
        for j in 1..=r[i] {
            let mut a = a.clone();
            a.push(j);
            s(i + 1, a.clone(), n, k, r.clone());
            a.pop();
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        r: [usize; n]
    }

    s(0, vec![], n, k, r);
}
