use proconio::input;

fn main() {
    input! {
        n:u64,
        v:[(i64,i64);n],
    }

    let ans: i64 = v.iter().map(|(x, y)| (x + y) * (y - x + 1) / 2).sum();
    println!("{}", ans);
}
