fn main() {
    proconio::input! {
        _n: i32,
        k: i32,
    }
    proconio::input! {
        s: String,
    }

    let mut count = 0;

    for i in s.chars() {
        if i == 'o' && count < k {
            count += 1;
            print!("o");
            continue;
        }

        dbg!(count <= k);

        print!("x");
    }

    println!("");
}
