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
       q: usize,
    }

    let mut h: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut b: Vec<Vec<usize>> = vec![vec![]; 2 * 100000 + 1];

    for _ in 0..q {
        input! {
            a: usize, i: usize,
        }

        if a == 1 {
            input! {
                j: usize,
            }
            h[j].push(i);
            b[i].push(j);
        }

        if a == 2 {
            h[i].sort();
            println!(
                "{}",
                h[i].iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }

        if a == 3 {
            b[i].sort();
            let mut tmp = vec![];
            let mut tep_num = 1000000;
            for i in b[i].clone() {
                if i == tep_num {
                    continue;
                }
                tmp.push(i);
                tep_num = i;
            }
            println!(
                "{}",
                tmp.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}
