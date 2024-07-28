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
        pear: [(isize, isize); n],
    }
    let mut x_sum = 0;
    let mut x = vec![0; n];
    for i in 0..n {
        x[i] = pear[i].0;
        x_sum += pear[i].0;
    }

    if x_sum > 0 {
        println!("No");
        return;
    }

    for i in 0..n {
        let a = min(0 - x_sum, pear[i].1 - pear[i].0);
        x[i] += a;
        x_sum += a;
    }

    if x_sum == 0 {
        println!("Yes");
        println!(
            "{}",
            x.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    } else {
        println!("No");
    }
}
