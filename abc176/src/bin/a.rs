use proconio::input;

fn main() {
    input! {
        n:u32,
        x:u32,
        t:u32
    }
    let tmp_ans: u32 = n / x;
    if n % x != 0 {
        println!("{}", (tmp_ans + 1) * t);
    } else {
        println!("{}", tmp_ans * t);
    }
}
