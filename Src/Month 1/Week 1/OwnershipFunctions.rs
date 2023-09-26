fn main() {
    let s = String::from("hello");

    // The ownership of s is transferred to the function and then back.
    let s = takes_and_gives_back(s);

    println!("The string is: {}", s);
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}
