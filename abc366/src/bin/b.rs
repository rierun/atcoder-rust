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
        mut s: [Chars; n],
    }

    s.reverse();

    let mut maxleight = 0;

    for i in 0..n {
        maxleight = max(maxleight, s[i].len());
    }
    // 最初に長さを揃える *で
    for i in 0..n {
        for _j in 0..maxleight - s[i].len() {
            s[i].push('*');
        }
    }

    // -90度回転
}

/* wa
    let mut maxleight = 0;

    for i in &s {
        maxleight = max(maxleight, i.len());
    }

    for i in 0..maxleight {
        dbg!(i);
        let mut cnt = 0;
        let mut fl = Vec::new();
        for j in 0..s.len() {
            if s[j].len() > i {
                if cnt >= 1 {
                    for _k in 0..cnt {
                        fl.push('*');
                    }
                }
                fl.push(s[j][i]);
            } else if i != n - 1 {
                cnt += 1;
            } else {
                fl.push('*');
            }
        }
        println!("{}", fl.iter().collect::<String>());
    }

*/
