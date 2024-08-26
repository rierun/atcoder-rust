#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};

fn aaaaa(k: u128, m: u128, a: &Vec<u128>) -> u128 {
    let mut sum: u128 = 0;
    dbg!(k);

    for i in a {
        dbg!(i);
        sum += min(i, &k);

        dbg!(sum);

        if sum > m {
            dbg!("hi");
            let ans = k - 1;
            return ans;
        }
    }
    return aaaaa(k + 1, m, a);
}

#[fastout]
fn main() {
    input! {
        n: usize, m: u128,
        a: [u128; n]
    }
    let a_sum: u128 = a.iter().sum();
    if a_sum > m {
        let ans = aaaaa(0, m, &a);
        println!("{}", ans);
    } else {
        println!("infinite")
    }
}
