use proconio::input;

fn main() {
    input! {
        w:usize,
        h:usize,
        n:usize,
        vec:[[usize;3];n]
    }

    let mut white_area = vec![vec![1; w]; h];

    for i in 0..n {
        let (x, y, a) = (vec[i][0], vec[i][1], vec[i][2]);
        match a {
            1 => {
                for j in 0..x {
                    for k in 0..h {
                        white_area[k][j] = 0;
                    }
                }
            }
            2 => {
                for j in x..w {
                    for k in 0..h {
                        white_area[k][j] = 0;
                    }
                }
            }
            3 => {
                for j in 0..w {
                    for k in 0..y {
                        white_area[k][j] = 0;
                    }
                }
            }
            4 => {
                for j in 0..w {
                    for k in y..h {
                        white_area[k][j] = 0;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut ans: i32 = 0;
    for i in 0..h {
        ans += white_area[i].iter().sum::<i32>();
    }

    println!("{}", ans);
}
