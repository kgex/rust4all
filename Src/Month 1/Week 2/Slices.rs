fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Create a slice
    let slice = &numbers[1..4]; // Elements at indices 1, 2, and 3

    for num in slice {
        println!("Number: {}", num);
    }
}
