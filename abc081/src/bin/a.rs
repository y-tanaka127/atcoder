use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }

    let mut ans = 0;
    for c in &s {
        let num: i32 = c.to_string().parse().unwrap();
        ans += num;
    }

    println!("{}", ans);
}
