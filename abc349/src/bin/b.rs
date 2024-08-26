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
       s: Chars
    }

    let mut c: Vec<usize> = vec![0; 128];

    for i in s.clone() {
        c[i as usize - 'a' as usize] += 1;
    }

    let mut d: Vec<usize> = vec![0; s.len() + 1];
    for i in c {
        d[i] += 1;
    }
    dbg!(&d);
    let mut ans = true;
    for i in 1..d.len() {
        if d[i] == 2 || d[i] == 0 {
            continue;
        } else {
            ans = false;
            break;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
