use trpl::StreamExt;

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
}
