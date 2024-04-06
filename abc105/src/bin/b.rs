use proconio::input;

fn main() {
    input! {
        n:i32,
    }

    let mut flag = false;

    for i in 0..=(n / 4) {
        for j in 0..=(n / 7) {
            if 4 * i + j * 7 == n {
                flag = true;
                break;
            }
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
