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
    input!{
        h: usize, w: usize,
        s: [Chars; h],
        mut plan: [(usize, usize, usize); h]
    }
}
