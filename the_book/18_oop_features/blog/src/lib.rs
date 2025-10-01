// Blog post
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // Create a new post
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Add a text content to the post
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Return a content of the post
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // Request a review
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review()) // Call trait's method
        }
    }

    // Approve the post
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve()) // Call trait's method
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Draft state
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Review is requested -> PendingReview state
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // Nothing happens
    }
}

// PendingReview state
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // Nothing happens
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Approved -> Published state
        Box::new(Published {})
    }
}

// Published state
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self // Nothing happens
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self // Nothing happens
    }

    // Only Published post returns the content
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
