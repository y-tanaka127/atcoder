use proconio::input;

fn main() {
    input! {
        n:usize,
    }

    let mut ans = 1;
    let mut count = 0;

    for i in 1..=n {
        if count < calc_dev_2(i) {
            count = calc_dev_2(i);
            ans = i;
        }
    }
    println!("{}", ans);
}

fn calc_dev_2(mut x: usize) -> usize {
    let mut count = 0;
    loop {
        if x % 2 == 0 {
            count += 1;
            x /= 2;
        } else {
            break;
        }
    }
    count
}
