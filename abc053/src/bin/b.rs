use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }

    let mut ans = 0;
    let mut flag: bool = false;
    let mut vec = vec![];

    for c in s {
        ans += 1;
        if c == 'A' && !flag {
            flag = true;
            ans = 1;
        } else if c == 'Z' && flag {
            vec.push(ans);
        }
    }
    vec.sort();
    println!("{}", vec[vec.len()-1]);
}
