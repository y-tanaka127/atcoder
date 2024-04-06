use proconio::input;

fn main() {
    input! {
        x:u32,
        y:u32
    }
    if y % 2 == 0 && x * 2 <= y && y <= x * 4 {
        println!("Yes")
    } else {
        println!("No");
    }
}
