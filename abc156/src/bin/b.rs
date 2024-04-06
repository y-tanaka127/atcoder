use proconio::input;

fn main() {
    input! {
        mut n:i32,
        k:i32
    }

    let mut ans = 0;

    while n != 0 {
        n /= k;
        ans += 1;
    }

    println!("{}", ans);
}
