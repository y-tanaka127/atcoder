fn main() {
    let array = [30, 20, 30];
    let mut sum = 0;
    for num in &array {
        sum += num;
    }
    assert_eq!(sum, 80);
}
