use proconio::input;

fn main() {
    input! {
        n:u32,
    }

    match n {
        1..=125 => println!("4"),
        126..=211 => println!("6"),
        212..=214 => println!("8"),
        _ => unreachable!(),
    }
}
