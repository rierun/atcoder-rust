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
    // 無向グラフ
    input! {
        n: usize, m: usize,
        a: [usize; n],
        edges: [(usize, usize, usize); m],
    }
}
