use proconio::input;

fn main() {
    input! {
        l:usize,
        n:usize,
        mut s:[String;n],
    }

    s.sort();

    for c in &s {
        print!("{}", c);
    }
    println!("");
}
