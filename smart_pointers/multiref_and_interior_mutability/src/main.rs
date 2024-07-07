use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nils};

fn main() {
    println!("Hello, world!");
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nils)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nils
}