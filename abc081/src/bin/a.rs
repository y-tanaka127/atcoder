use proconio::input;

fn main() {
    input! {
        s:i32,
    };
    let h = s / 100;
    let t = s % 100 / 10;
    let o = s % 10 / 1;
    let a = if h == 1 { 1 } else { 0 };
    let b = if t == 1 { 1 } else { 0 };
    let c = if o == 1 { 1 } else { 0 };
    println!("{}", a + b + c);
}
