use proconio::input;

fn main() {
    input! {
        n:i32,
    };
    let ans;
    if n % 2 == 0 {
        ans = "White";
    } else {
        ans = "Black";
    }
    println!("{}", ans);
}
