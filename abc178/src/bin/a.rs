use proconio::input;

fn main() {
    input! {
        x:i32,
    };
    let ans;
    if x == 0 {
        ans = 1;
    } else {
        ans = 0;
    }
    println!("{}", ans);
}
