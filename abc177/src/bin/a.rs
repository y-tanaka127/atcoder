use proconio::input;

fn main() {
    input! {
        d:i32,
        t:i32,
        s:i32,
    };
    let ans;
    if d / s <= t {
        ans = "Yes";
    } else {
        ans = "No";
    }

    println!("{}", ans);
}
