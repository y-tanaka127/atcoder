fn main() {
    let a = 100;
    let b = 111;

    for i in a..=b {
        let str_i = i.to_string();
        let chars_i = str_i.chars().rev().collect::<String>();

        if str_i == chars_i {
            println!("{}:true", i);
        } else {
            println!("{}:false", i);
        }
    }
}
