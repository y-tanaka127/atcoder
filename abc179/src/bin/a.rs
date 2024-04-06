use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s:Chars,
    }

    for c in &s {
        print!("{}", c);
    }
    if s[s.len() - 1] == 's' {
        println!("es");
    } else {
        println!("s");
    }
}
