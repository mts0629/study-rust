use blog::Post;

fn main() {
    let mut post = Post::new();

    /*
    // By using state pattern
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // Only published post returns its content
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    */

    // By using types
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
