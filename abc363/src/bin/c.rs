#[allow(unused_imports)]
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

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut s: Chars,
    }
    s.sort();

    let mut cnt = 0;
    for perm in (0..n).permutations(n) {
        let st = perm.iter().map(|&i| s[i]).collect::<Vec<char>>();

        dbg!(st.clone());
        let mut ok = true;
        for i in 0..n - k + 1 {
            let mut pallindrome = true;
            for j in 0..k {
                pallindrome &= st[i + j] == st[i + k - j - 1];
            }
            ok &= !pallindrome;
        }
        cnt += ok as usize;
    }

    println!("{}", cnt);
}
/*
        if a.contains(&perm) {
            continue;
        }
        a.push(perm.clone());

        dbg!(perm.clone());
        let mut ok = true;
        for i in 0..n - k + 1 {
            let mut pallindrome = true;
            for j in 0..k {
                pallindrome &= perm[i + j] == perm[i + k - j - 1];
            }
            ok &= !pallindrome;
        }
        cnt += ok as usize;


*/
