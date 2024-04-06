use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i32;3 * n]
    }

    a.sort();
    a.reverse();

    println!("{:?}", a);
    for i in 0..n {
        println!("{}:{}", i, a[3 * i + 1]);
    }
    // let mut sum = 0;

    // for i in 0..n {
    //     sum += a[3 * i + 1];
    // }

    // println!("{}", sum);
}
