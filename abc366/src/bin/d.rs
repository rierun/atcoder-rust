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
    }

    let mut a = vec![vec![vec![0; n + 1]; n + 1]; n + 1];

    for i in 0..n {
        for j in 0..n {
            input! {
                aa: [usize; n],
            }
            for k in 0..n {
                a[i][j][k] = aa[k];
            }
        }
    }
    dbg!(&a);

    input! {
        q: usize,
    }

    for _qq in 0..q {
        dbg!(_qq);
        input! {
            lx: usize, rx: usize, ly: usize, ry: usize, lz: usize, rz: usize,
        }
        let lx = lx - 1;
        let ly = ly - 1;
        let lz = lz - 1;
        let rx = rx - 1;
        let ry = ry - 1;
        let rz = rz - 1;

        let mut sum = 0;
        for i in lx..=rx {
            for j in ly..=ry {
                for k in lz..=rz {
                    dbg!(i);
                    dbg!(j);
                    dbg!(k);
                    dbg!(a[i][j][k]);
                    sum += a[i][j][k];
                }
            }
        }
        println!("{}", sum);
    }
}
