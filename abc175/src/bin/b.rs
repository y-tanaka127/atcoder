use proconio::input;

fn main() {
    input! {
        n:u64,
        mut l:[u64;n],
    }

    let tmp_l_slice = &mut l;
    tmp_l_slice.sort();
    let mut ans = 0;

    for i in 0..tmp_l_slice.len() {
        for j in 0..i {
            for k in 0..j {
                if tmp_l_slice[i] != tmp_l_slice[j]
                    && tmp_l_slice[j] != tmp_l_slice[k]
                    && tmp_l_slice[i] < tmp_l_slice[k] + tmp_l_slice[j]
                {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
