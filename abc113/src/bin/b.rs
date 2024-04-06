use proconio::input;

fn main() {
    input! {
        n:usize,
        t:f64,
        a:f64,
        h:[f64;n],
    };

    let mut tmp_tmpareture = 999.999;
    let mut ans = 0;

    for i in 0..n {
        if tmp_tmpareture > calc_temp(t, h[i], a) {
            tmp_tmpareture = calc_temp(t, h[i], a);
            ans = i;
        }
    }

    println!("{}", ans + 1);
}

// 気温との差算出
fn calc_temp(t: f64, x: f64, a: f64) -> f64 {
    let tempareture = (t - x * 0.006) - a;
    tempareture.abs()
}
