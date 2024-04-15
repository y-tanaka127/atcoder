use proconio::input;

fn main() {
    input! {
        n:f64,
    }

    let mut ans = 99999999;
    let tmp_n = n.sqrt().floor();

    for a in 1..=tmp_n as i64 {
        let b = n as i64 / a;
        if n as i64 % a == 0 {
            let tmp_ans = a.to_string().len().max(b.to_string().len());
            ans = tmp_ans.min(ans);
        }
    }
    println!("{}", ans);
}
