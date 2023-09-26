fn main() {
    // Create a String
    let mut s = String::from("Hello, ");

    // Append to the string
    s.push_str("world!");

    // Concatenate strings
    let s2 = " Rust";
    s.push_str(s2);

    println!("{}", s);
}
