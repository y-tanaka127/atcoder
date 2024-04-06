use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        n:Bytes,
    }

    let mut tmp_sum = 0;
    for c in &n {
        let tmp_num = c - b'0';
        tmp_sum += tmp_num as i32;
    }

    if tmp_sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
