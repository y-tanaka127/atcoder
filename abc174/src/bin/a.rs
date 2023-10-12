use proconio::input;

fn main() {
    input! {
        x:i32,
    };
    let ans;
    if x < 30 {
        ans = "No";
    } else {
        ans = "Yes";
    }
    println!("{}", ans);
}
