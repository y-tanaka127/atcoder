use proconio::input;

fn main() {
    input! {
        n:i32,
    }

    for i in 0..=9 {
        for j in 0..=9 {
            if i * j == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
