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
        a: [usize; 9],
        b: [usize; 8],
    }

    let a_sum = a.iter().sum::<usize>() + 1;
    let b_sum = b.iter().sum::<usize>();

    println!("{}", a_sum - b_sum);
}
