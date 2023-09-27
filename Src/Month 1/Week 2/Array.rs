fn main() {
    // Create an array of integers
    let numbers = [1, 2, 3, 4, 5];

    // Access elements
    let first = numbers[0];
    println!("First element: {}", first);

    // Iterate through the array
    for num in &numbers {
        println!("Number: {}", num);
    }
}
