use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new channel
    let (tx, rx) = mpsc::channel(); // tx: transmitter, rx: receiver

    // Send a value from a thread
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // Receive a value from the thread
    let received = rx.recv().unwrap(); // recv blocks execution
    println!("Got: {received}");

    let (tx, rx) = mpsc::channel();

    // Send multiple values from a thread
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            // Wait 1[sec] for every sending
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Receive multiple values by an iterator
    for received in rx {
        println!("Got: {received}");
    }

    let (tx, rx) = mpsc::channel(); // mpsc: multiple producer, single consumer

    // Clone a transmitter
    // Values are sent to the same recevier rx with the original transmitter tx
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // All values are caught
    for received in rx {
        println!("Got: {received}");
    }
}
