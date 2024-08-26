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
       mut a: [usize; n],
    }

    let mut adr = vec![0; n + 1];
    for i in 0..n {
        adr[a[i]] = i;
    }

    let mut ans = vec![[0; 2]];

    for i in 0..n {
        if a[i] != i + 1 {
            let j = adr[i + 1];
            ans.push([min(i, j), max(i, j)]);
            let tmp = a[i];
            a[i] = a[j];
            a[j] = tmp;
            adr[a[i]] = j;
            adr[a[j]] = i;
        }
    }

    println!("{}", ans.len() - 1);
    for i in 1..ans.len() {
        println!("{} {}", ans[i][0] + 1, ans[i][1] + 1);
    }
}
