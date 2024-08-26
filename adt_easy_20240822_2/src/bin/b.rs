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
       s: String,
    }
    let con = ["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

    for i in con {
        if i == s {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
