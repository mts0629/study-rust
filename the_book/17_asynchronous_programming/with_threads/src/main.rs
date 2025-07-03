use std::{thread, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

// Return an interval count with thread API
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    thread::spawn(move || {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_millis(1));
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
    // async with thread APIs
    trpl::run(async {
        let mut stream = get_intervals().take(10);

        while let Some(interval) = stream.next().await {
            println!("{interval}");
        }
    });

    println!("***");

    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
