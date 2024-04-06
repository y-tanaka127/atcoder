use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }

    let mut price = 700;

    for c in &s {
        if c.to_string() == "o" {
            price += 100;
        }
    }
    println!("{}", price);
}
