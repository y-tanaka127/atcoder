use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }

    a.sort();

    let ans = a.iter().last().unwrap() - a.iter().next().unwrap();
    println!("{}", ans);
}
