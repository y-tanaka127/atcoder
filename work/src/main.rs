fn main() {
    println!("{}", fact(6));
}
fn fact(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        fact(n - 1) * n
    }
}
