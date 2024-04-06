use proconio::input;

fn main() {
    input! {
        s:String,
    }

    let start_s = s.chars().next().unwrap();
    let len = s.chars().count();
    let end_s = s.chars().last().unwrap();
    println!("{}{}{}", start_s, len - 2, end_s);
}
