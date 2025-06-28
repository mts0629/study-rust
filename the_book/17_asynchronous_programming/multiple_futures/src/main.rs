use std::future::Future;
use std::pin::{pin, Pin};
use std::thread;
use std::time::Duration;
use trpl::Either;

// Wait N ms
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms} ms");
}

// Timeout with async
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    // trpl::race starts polling from the first arguments (future),
    // so pass future_to_try firstly
    // to make it to get a chance to complete the task when max_time is very short
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // Each async block has a unique enum,
        // so wrap async blocks by Pin to line up in a vector
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, tx_fut, rx_fut];

        // Join multiple futures with same Output type
        trpl::join_all(futures).await;
    });

    println!("***");

    trpl::run(async {
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        // Join multiple futures with different (and distinct) types
        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });

    println!("***");

    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        // Race two futures and get the first finished one
        trpl::race(slow, fast).await;
    });

    println!("***");

    trpl::run(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30); // thread::sleep blocks future a
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await; // And control is back to runtime here
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75); // thread::sleep blocks future b
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await; // And control is back to runtime here
            println!("'b' finished.");
        };

        // It awaits futures after execution of all slow function
        trpl::race(a, b).await;
    });

    println!("***");

    trpl::run(async {
        // Handling off control for every yield_now
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        // Awaits for each yield_slow
        trpl::race(a, b).await;
    });

    println!("***");

    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "I finished!"
        };

        // Timeout function with async building blocks
        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                // slow will be timeout
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
