use proconio::input;

fn main() {
    input! {
        k:i32,
        a:i32,
        b:i32,
    };
    let mut ans = "NG";
    let mut i = a;
    while i <= b {
        if i % k == 0 {
            ans = "OK";
            break;
        }
        i += 1;
    }
    println!("{}", ans);
}
