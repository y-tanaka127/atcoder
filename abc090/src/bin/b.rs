use proconio::input;

fn main() {
    input! {
        a:i32,
        b:i32
    }

    let mut ans = 0;

    for i in a..=b {
        let str_i = i.to_string();
        let rev_str_i = str_i.chars().rev().collect::<String>();

        if str_i == rev_str_i {
            ans += 1;
        }
    }
    println!("{}", ans);
}
