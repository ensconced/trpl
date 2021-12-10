fn main() {
    for x in 1..=10 {
        println!("{}", fib(x));
    }
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
