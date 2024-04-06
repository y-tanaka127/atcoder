use proconio::input;

fn main() {
    input! {
        n:i32,
        y:i32,
    }

    for x_10000 in 0..=n {
        for y_5000 in 0..n {
            let z_1000 = n - (x_10000 + y_5000);
            if z_1000 >= 0 && y == x_10000 * 10000 + y_5000 * 5000 + z_1000 * 1000 {
                println!("{} {} {}", x_10000, y_5000, z_1000);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
