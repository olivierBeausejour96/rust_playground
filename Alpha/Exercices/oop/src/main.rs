extern crate oop;
use oop::*;
use oop::{Post};

fn main() {
    let mut post = Post::new();
    post.add_text("Hello world!");
    post.request_review();
    post.approve();
    post.approve();
    println!("public: {}", post.content());
}