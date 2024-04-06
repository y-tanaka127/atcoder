use proconio::input;

fn main() {
    input! {
        n:u64,
        d:f64,
        v:[(f64,f64);n],
    }

    let mut ans = 0;
    for (x, y) in &v {
        let tmp_d = (x * x + y * y).sqrt();
        if tmp_d <= d {
            ans += 1;
        }
    }
    println!("{}", ans);
}
