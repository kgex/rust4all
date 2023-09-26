use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a channel
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread that sends data to the main thread
    thread::spawn(move || {
        let message = String::from("Hello from the spawned thread!");
        sender.send(message).unwrap();
    });

    // Receive the message from the spawned thread
    let received_message = receiver.recv().unwrap();
    println!("Received: {}", received_message);
}
