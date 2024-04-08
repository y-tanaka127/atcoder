use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut a:[i32;n]
    }

    let mut ans = 0;

    a.sort();

    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for num in a {
        let count = hash_map.entry(num).or_insert(0);
        *count += 1;
    }

    println!("{:?}", hash_map);
}
