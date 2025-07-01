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
            trpl::sleep(Duration::from_millis(time_to_sleep));

            tx.send(format!("Message: '{message}'")).unwrap();
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
}
