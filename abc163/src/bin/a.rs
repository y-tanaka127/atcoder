use proconio::input;

fn main() {
    input! {
        r:f64,
    }

    println!("{}", 2 as f64 * r * std::f64::consts::PI);
}
