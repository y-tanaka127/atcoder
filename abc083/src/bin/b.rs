use proconio::input;

fn main() {
    input! {
        n:i32,
        a:i32,
        b:i32
    }

    let mut sum: i32 = 0;

    for i in 1..=n {
        let mut tmp_sum: i32 = 0;
        let mut tmp_i = i;

        while tmp_i > 0 {
            tmp_sum += tmp_i % 10;
            tmp_i /= 10;
        }

        if tmp_sum >= a && tmp_sum <= b {
            sum += i;
        }
    }

    println!("{}", sum);
}
