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
        h: isize , w: isize,
        sy: isize, sx: isize,
        c: [(Chars); h],
        x: Chars,
    }

    let mut nx: isize = sx - 1;
    let mut ny: isize = sy - 1;

    for i in x {
        dbg!(i);
        if i == 'U' {
            if ny > 0 && c[(ny - 1) as usize].0[nx as usize] == '.' {
                ny -= 1;
                dbg!("Do");
            }
        } else if i == 'D' {
            if ny < h - 1 && c[(ny + 1) as usize].0[nx as usize] == '.' {
                ny += 1;
                dbg!("Do");
            }
        } else if i == 'L' {
            if nx > 0 && c[ny as usize].0[(nx - 1) as usize] == '.' {
                nx -= 1;
                dbg!("Do");
            }
        } else if i == 'R' {
            if nx < w - 1 && c[ny as usize].0[(nx + 1) as usize] == '.' {
                nx += 1;
                dbg!("Do");
            }
        } else {
            panic!();
        }
    }

    println!("{} {}", ny + 1, nx + 1);
}
