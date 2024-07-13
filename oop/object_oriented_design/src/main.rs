use blog::Post;
use object_oriented_design::blog;

fn main() {
    println!("Hello, world!");


    let mut post = Post::new();
    // draft
    post.add_text("I ate yummy salad");
    assert_eq!("", post.content());
    post.add_text(" hi");
    // pending review
    post.request_review();
    assert_eq!("", post.content());


    post.approve();
    // pending review
    post.approve();
    // approved
    post.add_text("hi");
    assert_eq!("I ate yummy salad hi", post.content());
}
