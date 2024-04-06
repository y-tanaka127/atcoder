use proconio::input;

fn main() {
    input! {
        n:usize,
        z:i32,
        mut m:[i32;n],
    }
    let total_m: i32 = m.iter().sum();

    let useful_z = z - total_m;

    m.sort();

    let ans = n as i32 + (useful_z / m[0]);
    println!("{}", ans);
}
