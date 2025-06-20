use std::sync::Mutex;
use std::thread;

fn main() {
    // Create a mutex
    let m = Mutex::new(5);

    {
        // Acquire a mutex's lock and get a mutable reference
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}"); // Show a value and "poisoning"

    // Compilation failed,
    // because ownership of a mutex's lock can't be moved into multiple threads
    /*let counter = Mutex::new(0);
    let mut handles = vec![];

    // Count up the mutex in 10 threads
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            // value moved into closure
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());*/
}
