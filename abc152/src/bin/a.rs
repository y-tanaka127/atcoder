use proconio::input;

fn main() {
    input! {
        n:i32,
        m:i32,
    };
    let ans;
    if n == m {
        ans = "Yes";
    } else {
        ans = "No";
    }
    println!("{}", ans);
}
