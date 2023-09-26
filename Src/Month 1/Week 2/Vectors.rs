fn main() {
    // Create a vector of integers
    let mut numbers: Vec<i32> = Vec::new();

    // Add elements to the vector
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Access elements
    let first = numbers[0];
    println!("First element: {}", first);

    // Iterate through the vector
    for num in &numbers {
        println!("Number: {}", num);
    }
}
