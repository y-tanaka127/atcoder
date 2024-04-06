use proconio::input;

fn main() {
    input! {
        n:i64,
        x:[i64;n],
    }
    let manhattan: i64 = x.iter().map(|ele| ele.abs()).sum();
    let euclid = (x.iter().map(|ele| ele * ele).sum::<i64>() as f64).sqrt();
    let chebishef: i64 = x.iter().map(|ele| ele.abs()).max().unwrap();
    println!("{}", manhattan);
    println!("{:?}", euclid);
    println!("{}", chebishef);
}
