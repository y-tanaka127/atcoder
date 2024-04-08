use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        vec:[(usize,usize);m],
    }

    let mut count_vec: Vec<usize> = vec![0; n];

    for i in 0..m {
        let num: (usize, usize) = vec[i];
        count_vec[num.0 - 1] += 1;
        count_vec[num.1 - 1] += 1;
    }
    for i in 0..n {
        println!("{}", count_vec[i]);
    }
}
