use std::time::Duration;

fn main() {
    // Multi-task with async
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });

    println!("***");

    // Join futures
    // execution is the exect same order every time by runtime's control
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });

    println!("***");

    // Async channel
    trpl::run(async {
        let (tx, mut rx) = trpl::channel(); // Receiver is mutable

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    });

    println!("***");

    // Async multi channels
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone(); // cloned before move
        let tx1_fut = async move {
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
        }; // tx1 is droped here, trigger rx closing

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                // rx closes when senders are dropped
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
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
        }; // tx is dropped here, trigger rx closing

        // Join channels
        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}
