use proconio::input;

fn main() {
    input! {
        n:u64,
        a:[u64;n],
    }
    let mut height: u64 = 0;
    let mut sum: u64 = 0;
    for i in &a {
        if height >= *i {
            sum += height - i;
        } else if height < *i {
            height = *i
        }
    }
    println!("{}", sum);
}
