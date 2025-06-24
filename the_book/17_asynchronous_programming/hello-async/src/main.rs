// trpl: support crate of "the book"
use trpl::{Either, Html};

// Return a text of the title element from requested page URL
// executed asynchronously
async fn page_title(url: &str) -> Option<String> {
    // Await a response and its text
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

// Same definition
// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let response_text = trpl::get(url).await.text().await;
//         Html::parse(&response_text)
//             .select_first("title")
//             .map(|title_element| title_element.inner_html())
//     }
// }

async fn page_url_and_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    /*let url = &args[1];

    trpl::run(async {
        // async functions must be called in async blocks
        match page_title(url).await {
            Some(title) => println!("The title of {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })*/

    // Print the title of the first-returned URL from 2 URLs
    trpl::run(async {
        let title_fut_1 = page_url_and_title(&args[1]);
        let title_fut_2 = page_url_and_title(&args[2]);

        // Get the first-returned value of two by race
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}
