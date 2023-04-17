use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a channel with a sender and receiver
    let (sender, receiver) = mpsc::channel();

    // Spawn five threads
    for i in 0..5 {
        let sender = sender.clone();
        thread::spawn(move || {
            // Send a message (in this case, an integer) through the channel
            sender.send(i).unwrap();
        });
    }

    // Receive messages from the channel
    let mut received = vec![];
    for _ in 0..5 {
        let num = receiver.recv().unwrap();
        received.push(num);
    }

    println!("Received messages: {:?}", received);
}