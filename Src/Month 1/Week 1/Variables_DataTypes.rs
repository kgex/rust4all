fn main() {
    // Variables are immutable by default
    let x = 5;
    println!("The value of x is: {}", x);

    // To make a variable mutable, use the 'mut' keyword
    let mut y = 10;
    y = 20;
    println!("The value of y is: {}", y);

    // Data types
    let a: i32 = 42;  // 32-bit signed integer
    let b: f64 = 3.14;  // 64-bit floating-point number
    let c: char = 'A';  // Unicode character
    let d: bool = true;  // Boolean
}
