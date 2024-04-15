use proconio::input;

fn main() {
    input! {
        c:[[u32;3];3],
    }

    let sum1 = c[0][0] + c[1][1] + c[2][2];
    let sum2 = c[0][1] + c[1][2] + c[2][0];
    let sum3 = c[0][2] + c[1][0] + c[2][1];

    if (sum1 == sum2) && (sum2 == sum3) && (sum3 == sum1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
