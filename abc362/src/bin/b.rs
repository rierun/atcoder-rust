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
        xa: isize, ya: isize,
        xb: isize, yb: isize,
        xc: isize, yc: isize,
    }

    // 辺ab ac bcの長さを求める
    let ab = (xa - xb) * (xa - xb) + (ya - yb) * (ya - yb);
    let ac = (xa - xc) * (xa - xc) + (ya - yc) * (ya - yc);
    let bc = (xb - xc) * (xb - xc) + (yb - yc) * (yb - yc);

    // 三平方の定理

    dbg!(ab, ac, bc);

    let mut ans = "No";
    if ab + ac == bc {
        ans = "Yes"
    }
    if ac + bc == ab {
        ans = "Yes"
    }
    if bc + ab == ac {
        ans = "Yes"
    }

    println!("{}", ans);
}
