use proconio::input;

fn main() {
    input! {
        s:i32,
        w:i32,
    };
    let ans;
    if s <= w {
        ans = "unsafe";
    } else {
        ans = "safe";
    }
    println!("{}", ans);
}
