// Media aggregator library
use core::fmt::Debug;

// Trait
// Common summarization interface
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// News article
#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
// If empty, default implementation is used
// impl Summary for NewsArticle {}

// Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Summary trait for Tweet
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // Default implementation is called
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

// Use traits as a parameter
pub fn notify(item: &impl Summary) {
    // Syntax suger of trait bound syntax:
    // pub fn notify<T: Summary>(item: &T) { ... }
    println!("Breaking news! {}", item.summarize())
}

// Using multiple trait bounds
pub fn notify_debug(item: &(impl Summary + Debug)) {
    // = pub fn notify<T: Summary + Debug>(item: &T) { ... }
    println!("{:?}", item);
    println!("Breaking news! {}", item.summarize())
}

// Trait bounds with where clauses
fn _some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Summary + Debug,
    U: Clone + Debug,
{ 42 }

// Return types that implement traits
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Pair struct
struct _Pair<T> {
    x: T,
    y: T,
}

// _new method is always implemented to _Pair
impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

use std::fmt::Display;

// Conditionally implementation:
// applied only for the type which implements Display trait and PartialOrd trait
impl<T: Display + PartialOrd> _Pair<T> {
    // Display enables printing and PartialOrd enables comparison
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Blanket implementation
// impl<T: Display> ToString for T {
//     ...
// }
