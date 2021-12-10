fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(num: i32) -> i32 {
    num + 1
}
