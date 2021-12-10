fn main() {
    let s1 = gives_ownership();
    println!("{}", s1); // main is the owner of this string now...
    let s2 = String::from("hello");
    println!("{}", s2); // main also owns s2...
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
}

fn gives_ownership() {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
