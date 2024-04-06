use proconio::input;

fn main() {
    input! {
        k:u32,
        a:u32,
        b:u32
    }
    let c = b / k * k;

    if a <= c {
        println!("OK")
    } else {
        println!("NG")
    }
}
