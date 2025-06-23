// trpl: support crate of "the book"
use trpl::Html;

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];

    trpl::run(async {
        // async functions must be called in async blocks
        match page_title(url).await {
            Some(title) => println!("The title of {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
