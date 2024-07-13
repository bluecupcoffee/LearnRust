pub mod blog {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {
                    content_lock: false,
                })),
                content: String::new()
            }
        }

        pub fn request_review(&mut self) {
            let temp: String;
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
        pub fn add_text(&mut self, text: &str) {
            if !self.state.as_ref().unwrap().locked() {
                self.content.push_str(text);
            }
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    trait State {
        fn locked(&self) -> bool;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }



    struct Published {
        content_lock: bool
    }
    impl State for Published {
        fn locked(&self) -> bool {
            self.content_lock
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    struct PendingReview {
        content_lock: bool,
        second_approval: bool
    }
    impl State for PendingReview {
        fn locked(&self) -> bool {
            self.content_lock
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {
                content_lock: false,
            })
        }
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            if self.second_approval {
                Box::new(Published{
                    content_lock: true
                })
            } else {
                Box::new(PendingReview{
                    content_lock: true,
                    second_approval: true
                })
            }

        }
    }
    struct Draft {
        content_lock: bool
    }
    impl State for Draft {
        fn locked(&self) -> bool {
            self.content_lock
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview{
                content_lock: false,
                second_approval: false
            })
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
}

