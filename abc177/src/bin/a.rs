use proconio::input;

fn main() {
    input! {
        d:u32,
        t:u32,
        s:u32
    }

    if d / s > t {
        println!("No")
    } else if d / s < t {
        println!("Yes")
    } else {
        if d % s == 0 {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
