use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n:u64,
        mut x:i64,
        s:Chars
    }

    for c in &s {
        match c {
            "o" => x += 1,
            "x" => x -= 1,
            _ =>
        }
    }
    println!("{}",);
}
