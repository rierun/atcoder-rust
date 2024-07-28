fn main() {
    proconio::input! {
        n: i32,
        x: i32,
    }
    proconio::input! {
        a: [i32; n - 1],
    }

    let mut ma = a.clone();

    ma.sort();

    for i in 0..=100 {
        let mut a = a.clone();

        a.push(i);

        a.sort();

        let mut sum = 0;
        for j in 1..(n - 1) as usize {
            sum += a[j];
        }

        if sum >= x {
            println!("{}", i);
            return;
        }
    }

    println!("-1")
}
