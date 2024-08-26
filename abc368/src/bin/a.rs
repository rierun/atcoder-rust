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
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut ba: Vec<usize> = a[0..(n - k)].to_vec();
    dbg!(ba.clone());
    let mut bb: Vec<usize> = a[(n - k)..n].to_vec();
    dbg!(bb.clone());

    // baとbbを合体
    let mut b: Vec<usize> = Vec::new();
    b.append(&mut bb);
    b.append(&mut ba);

    // bを出力
    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
