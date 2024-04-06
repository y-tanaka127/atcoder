use proconio::input;

fn main() {
    input! {
        sx:f64,
        sy:f64,
        gx:f64,
        gy:f64
    }
    // gyは-1をかけて一直線になるようにする
    let a = (gy + sy) / (sx - gx);
    let b = sy - sx * ((sy + gy) / (sx - gx));

    let x = (-1.) * b / a;
    println!("{}", x);
}
