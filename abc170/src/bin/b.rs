use proconio::input;

fn main() {
    input! {
        x:i32,
        y:i32,
    };
    let ans;
    let a = (y - 2 * x) / 2;
    let b = x - a;
    if a >= 0 && b >= 0 && y % 2 == 0 {
        ans = "Yes";
    } else {
        ans = "No";
    }
    println!("{}", ans);
}
