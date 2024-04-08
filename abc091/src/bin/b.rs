use proconio::input;

fn main() {
    input! {
        n:usize,
        mut s:[String;n],
        m:usize,
        t:[String;m]
    }

    s.sort();

    let mut vec = Vec::new();

    for blue in &s {
        let mut ans = 0;

        for i in 0..s.len() {
            if blue == &s[i] {
                ans += 1;
            }
        }
        for j in 0..t.len() {
            if blue == &t[j] {
                ans -= 1;
            }
        }
        vec.push(ans);
    }

    vec.sort();

    let ans = if vec[vec.len() - 1] < 0 {
        0
    } else {
        vec[vec.len() - 1]
    };

    println!("{}", ans);
}
