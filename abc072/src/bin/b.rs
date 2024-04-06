use proconio::input;

fn main() {
    input! {
        s:String,
    }

    for i in 0..s.len() {
        if i % 2 == 0 {
            let tmp_s = s.chars().nth(i).unwrap();
            print!("{}",tmp_s);
        }
    }

    println!("");
}
