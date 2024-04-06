use proconio::input;

fn main() {
    input! {
        n:u64,
        k:u64,
        mut p:[u64;n],

    }

    let tmp_vec = &mut p;
    tmp_vec.sort();

    let mut ans: u64 = 0;
    for i in 0..k {
        ans += tmp_vec[i as usize];
    }
    println!("{}", ans);
}
