#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut post = Post::new();

        assert_eq!(post.content, "");
        
        let first_content = "Hello world!";
        post.add_text(first_content);
        assert_eq!(post.content(), "");

        post.request_review();
        assert_eq!(post.content(), "");

        post.approve();
        assert_eq!(post.content(), "");

        post.approve();
        assert_eq!(post.content(), first_content);

        post.add_text(first_content);
        assert_eq!(post.content(), first_content);
    }

    #[test]
    fn test_blog_v2() {
        use blog_v2::*;

        let content = "I like potatoes!!!!!!";

        let mut post = blog_v2::Post::new();

        post.add_text(content);

        let post = post.request_review();

        let post = post.approve();

        let post = post.approve();

        assert_eq!(post.content(), content);
    }
}


pub fn run() {
    println!("Hello world!");
}


pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove_last(&mut self) -> Option<i32> {
        let result = self.list.pop();
        if let Some(value) = result {
            self.update_average();
        }
        result
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}



pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(content) = self.state.as_ref().unwrap().add_content(text) {
            self.content.push_str(content);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn add_content<'a>(&self, content: &'a str) -> Option<&'a str> {
        None
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_content<'a>(&self, content: &'a str) -> Option<&'a str> {
        Some(content)
    }
}

struct PendingReview {
    approvals: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals < 1 {
            Box::new(PendingReview { approvals: self.approvals + 1 })
        }
        else {
            Box::new(Published {}) 
        }

    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }   
}

struct Published {}


impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

mod blog_v2 {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost_0 {
            PendingReviewPost_0 {
                content: self.content,
            }
        }
    }

    pub struct PendingReviewPost_0 {
        content: String,
    }

    impl PendingReviewPost_0 {
        pub fn approve(self) -> PendingReviewPost_1 {
            PendingReviewPost_1 {
                content: self.content,
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }

    //compile-time checked!!!
    //macros could be used for this to prevent code duplication!
    pub struct PendingReviewPost_1 {
        content: String,                
    }

    impl PendingReviewPost_1 {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }
}

