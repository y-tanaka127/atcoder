use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32;n],
    };
    let mut ans = 0;

    'outer: loop {
        for i in &mut a {
            if *i % 2 == 1 {
                break 'outer;
            }
            *i /= 2;
        }
        ans += 1;
    }
    println!("{}", ans);
}
