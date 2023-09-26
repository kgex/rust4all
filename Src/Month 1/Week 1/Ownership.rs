fn main() {
    // Ownership rule 1: Each value in Rust has a variable that's its "owner."
    let s1 = String::from("hello");

    // Ownership rule 2: You can only have one owner at a time.
    let s2 = s1; // s1 is no longer valid here; it's been "moved" to s2.

    // Ownership rule 3: When the owner goes out of scope, the value is dropped.
    // println!("s1: {}", s1); // This line will result in a compilation error.
    
    // However, s2 is still valid to use.
    println!("s2: {}", s2);
}
