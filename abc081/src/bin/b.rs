use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }

    let mut ans = 0;

    loop {
        a = a.iter().filter(|&x| x % 2 == 0).map(|x| x / 2).collect();
        if n == a.len() {
            ans += 1;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
