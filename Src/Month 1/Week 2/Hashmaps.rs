use std::collections::HashMap;

fn main() {
    // Create a HashMap
    let mut scores = HashMap::new();

    // Insert key-value pairs
    scores.insert("Alice", 95);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 90);

    // Access values
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None => println!("Alice's score not found"),
    }

    // Iterate through key-value pairs
    for (name, score) in &scores {
        println!("{} scored {}", name, score);
    }
}
