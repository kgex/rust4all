use std::thread;

fn main() {
    // Create a new thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: Count {}", i);
        }
    });

    // Main thread
    for i in 1..=3 {
        println!("Main: Count {}", i);
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
