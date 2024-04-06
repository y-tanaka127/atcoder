use proconio::input;

fn main() {
    input! {
        n:u32,
    }

    let ans = match n % 10_ {
        0 | 1 | 6 | 8 => "pon",
        2 | 4 | 5 | 7 | 9 => "hon",
        3 => "bon",
        _ => unreachable!(),
    };
    println!("{}", ans);
}
