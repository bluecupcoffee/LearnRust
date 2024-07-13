use encoding_states_and_behaviors_as_types::blog::*;

fn main() {
    println!("Hello, world!");

    let mut post = Post::new();

    post.add_text("I ate a yummy salad");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a yummy salad", post.content());
}
