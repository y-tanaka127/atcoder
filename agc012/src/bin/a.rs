use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;3 * n]
    }

    a.sort();
    a.reverse();

    let mut sum: i64 = 0;

    for i in 0..n {
        sum += a[2 * i + 1];
    }

    println!("{}", sum);
}
