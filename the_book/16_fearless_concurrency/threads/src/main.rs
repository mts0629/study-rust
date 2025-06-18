use std::thread;
use std::time::Duration;

fn main() {
    // Create a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait finishing of the spawned thread
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // A thread must have an ownership of captured value by move
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // drop(v); // Cause compile error, because of violating the ownership rules

    handle.join().unwrap();

    // All spawned thread are shut down when the main thread completes
}
