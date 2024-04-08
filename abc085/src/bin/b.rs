use proconio::input;

fn main() {
    input! {
        n:usize,
        mut d:[u32;n],
    }

    d.sort();

    let mut ans = 1;
    let mut mochi = d[0];

    for i in 0..n {
        if mochi == d[i] {
            continue;
        } else {
            ans += 1;
            mochi = d[i];
        }
    }
    println!("{}", ans);
}
