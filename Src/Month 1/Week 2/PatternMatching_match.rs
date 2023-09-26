fn main() {
    let number = 42;

    match number {
        0 => println!("It's zero."),
        1..=100 => println!("It's between 1 and 100."),
        _ => println!("It's something else."), // Default case
    }

    let name = "Alice";

    match name {
        "Alice" => println!("Hello, Alice!"),
        "Bob" => println!("Hello, Bob!"),
        _ => println!("Hello, stranger!"),
    }
}
