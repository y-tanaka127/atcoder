use proconio::input;

fn main() {
    input! {
        s:u32,
        w:u32
    }

    if s > w {
        println!("safe")
    } else {
        println!("unsafe")
    }
}
