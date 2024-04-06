use proconio::input;

fn main() {
    input! {
        n:i32
    }

    let mut ans = 100000;

    for mut a in 1..n {
        let mut b = n - a;

        let mut sum_a = 0;

        while a > 0 {
            sum_a += a % 10;
            a /= 10;
        }

        let mut sum_b = 0;

        while b > 0 {
            sum_b += b % 10;
            b /= 10;
        }

        if sum_a + sum_b < ans {
            ans = sum_a + sum_b;
        }
    }
    println!("{}", ans);
}
