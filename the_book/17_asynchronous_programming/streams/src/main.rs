use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

// Return a stream of String
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            // Sleep 100 ms/300 ms by turns
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

// Return an interval count
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            // Sleep 1[ms] and add a count
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // Iterate and double values
        let iter = values.iter().map(|n| n * 2);
        // Stream for asynchronous iteration
        let stream = trpl::stream_from_iter(iter);
        // Filter only passes mulitple of 3 or 5
        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });

    println!("***");

    trpl::run(async {
        let mut messages = get_messages();

        // Get messages from stream
        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });

    println!("***");

    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        // Timeout occurs while getting messages from stream, but it don't stop
        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });

    println!("***");

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}")) // Adjust the type with messages
            .throttle(Duration::from_millis(100)) // Limit the rate for every 100[ms]
            .timeout(Duration::from_secs(10)); // Set a long duration to avoid timeout
        let merged = messages.merge(intervals).take(20); // Stop after pulling 20 items
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}
