use std::ops::Deref;

fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let name = String::from("Jon Stunger");
    // doesnt work
    // hello(name);
    hello(&name[..]);
    let nam = MyBox::new(String::from("John Bunger"));
    hello(&nam);
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}