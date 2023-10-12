use proconio::input;

fn main() {
    input! {
        r:f64,
    };
    let pi = std::f64::consts::PI;
    let ans = r * 2.0 * pi;
    println!("{}", ans);
}
