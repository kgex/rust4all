fn main() {
    let s1 = String::from("hello");
    
    // Borrowing rule 1: You can have multiple immutable references (&) to a value.
    let len = calculate_length(&s1);

    // Borrowing rule 2: You can't have both a mutable and an immutable reference in the same scope.
    // let r1 = &mut s1; // This line will result in a compilation error.

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't have ownership, so nothing happens.
