use proconio::input;

fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        x:i32,
        y:i32
    }

    let max_amount = if x > y { x } else { y };
    let mut ans = c * 2 * max_amount;

    for i in 0..=max_amount {
        let tmp_c = i * 2;
        let tmp_a = if x - i >= 0 { x - i } else { 0 };
        let tmp_b = if y - i >= 0 { y - i } else { 0 };
        let tmp_cost = tmp_a * a + tmp_b * b + tmp_c * c;
        ans = ans.min(tmp_cost);
    }
    println!("{}", ans);
}
