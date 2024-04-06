use proconio::input;

fn main() {
    input! {
        n:usize,
        mut x:i32,
        mut a:[i32;n]
    }

    a.sort();

    let mut count = 0;

    for i in 0..n {
        x -= a[i];

        if (x < 0) || (x > 0 && i == n - 1) {
            break;
        } else if x == 0 {
            count += 1;
            break;
        } else {
            count += 1;
        }
    }
    println!("{}", count);
}
