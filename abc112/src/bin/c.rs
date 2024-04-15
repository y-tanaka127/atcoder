use proconio::input;

fn main() {
    input! {
        n:usize,
        vec:[[u64;3];n]
    }

    println!("{:?}", vec);
}
