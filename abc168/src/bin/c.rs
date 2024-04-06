use proconio::input;

fn main() {
    input! {
        a:f64,
        b:f64,
        h:f64,
        m:f64
    }

    let angle_h = 360. / 12. * h;
    let angle_m = 360. / 60. * m;
    let tmp_angle = angle_h - angle_m;
    let angle: f64;

    if tmp_angle.abs() < 180. {
        angle = tmp_angle.abs();
    } else {
        angle = 360. - tmp_angle.abs();
    }
    let ans = a * a + b * b - 2. * a * b * angle.cos();
    println!("{}", ans);
}
