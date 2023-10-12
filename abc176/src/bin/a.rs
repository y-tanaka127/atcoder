use proconio::input;

fn main() {
    input!{
        n:i32,
        x:i32,
        t:i32,
    };
    let ans:i32;
    if n % x == 0 {
        ans = n / x * t;
    }else{
        ans = ((n / x) + 1) * t;
    }
    println!("{}",ans);
}
