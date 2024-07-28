use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize, x: usize,
        a: [Usize1; n],
    }

    //
    let mut a = a;
    a.insert(k, x - 1);

    println!(
        "{}",
        a.iter()
            .map(|&x| x + 1)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
