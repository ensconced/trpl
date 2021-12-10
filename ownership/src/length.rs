fn main() {
    let s1 = String::from("hello");
    let (s2, length) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, length);
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}
