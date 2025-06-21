// use std::rc::Rc;
use std::sync::{Arc, Mutex};
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

    // Compilation failed
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

    // Compilation failed
    // thread-unsafe, access to counter may be interrupted by other threads
    /* let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);

        for handle in handles {
            handle.join().unwrap();
        }
    }

    println!("Result: {}", *counter.lock().unwrap()); */

    // Create an atomic reference count
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Thread-safe access
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
