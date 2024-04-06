use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
        x:[[i32;d];n],
    }

    let mut ans = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            let mut total_d: f64 = 0.0;
            for k in 0..d {
                let tmp_d = (x[i][k]) - (x[j][k]);
                total_d += (tmp_d * tmp_d).abs() as f64;
            }
            if total_d.sqrt() % 1.0 == 0.0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
