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

    let mut vec:Vec<(&i32,&i32)> = hash_map.iter().collect();
    vec.sort_by(|a,b| a.1.cmp(&b.1));

    if vec.len() > k {
        for i in 0..(vec.len() - k){
            ans += vec[i].1;
        }
    }
    println!("{}", ans);
}
