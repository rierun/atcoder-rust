use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    //
    let mut a = a;
    a.sort();

    let mut ans_min = std::usize::MAX;
    for i in 0..k + 1 {
        let min = a[i];
        let max = a[i + n - k - 1];
        let ans = max - min;
        if ans < ans_min {
            ans_min = ans;
        }
    }

    println!("{}", ans_min);
}
