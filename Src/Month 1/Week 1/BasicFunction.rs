fn main() {
    // Variables and Data Types
    let my_integer: i32 = 42;
    let my_float: f64 = 3.14;
    let my_boolean: bool = true;
    let my_character: char = 'A';
    let my_string: String = String::from("Hello, Rust!");

    println!("Integer: {}", my_integer);
    println!("Float: {}", my_float);
    println!("Boolean: {}", my_boolean);
    println!("Character: {}", my_character);
    println!("String: {}", my_string);

    // Control Flow - if-else
    let number = 25;
    if number < 50 {
        println!("Number is less than 50.");
    } else {
        println!("Number is greater than or equal to 50.");
    }

    // Functions
    let result = add_numbers(5, 7);
    println!("The sum is: {}", result);
}

// A simple function that adds two numbers
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
