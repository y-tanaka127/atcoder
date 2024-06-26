use proconio::input;

fn main() {
    input! {
        s:String,
    }

    let alphabet = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];

    for i in 0..alphabet.len() {
        if !s.contains(alphabet[i]) {
            println!("{}", alphabet[i]);
            return;
        }
    }
    println!("None");
}
