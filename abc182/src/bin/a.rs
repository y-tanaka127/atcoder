use proconio::input;

fn main() {
    input! {
        a:u32,
        b:u32,
    }

    println!("{}", 2 * a + 100 - b);
}
