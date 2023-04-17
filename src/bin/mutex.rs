use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Create a mutable counter
    let counter = Arc::new(Mutex::new(0));

    // Spawn multiple threads that increment the counter
    let mut handles = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}