use proconio::input;

fn main() {
    input! {
        s:String,
        t:String,
    }

    let mut tmp_s = s.chars().collect::<Vec<_>>();
    tmp_s.sort();
    let sorted_s: String = tmp_s.into_iter().collect();

    let mut tmp_t = t.chars().collect::<Vec<_>>();
    tmp_t.sort();
    let sorted_t: String = tmp_t.into_iter().rev().collect();

    if sorted_s < sorted_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
