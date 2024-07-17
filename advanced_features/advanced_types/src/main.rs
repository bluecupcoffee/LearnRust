use std::fmt::Pointer;

// can use newtype pattern to abstract away implementation details of a type
// exposes a public API different from the private inner API of the type

// creating type synonyms with type aliases

// main use for synonyms is to reduce repetition

// never type that never returns
// panic is a never return type
// loop is a never return type

// dynamically sized types and the sized trait
// to use dynamically sized types you need to provide Rust with
// a Sized trait to determine whether the type's size is known
// at compile time

//fn generic<T>(t: T) {} is actually the following
fn generic<T: Sized>(t: T) {

}

// generic functions only work with sizes known at compile time
// you can use the following to relax the restriction
fn generic2<T: ?Sized>(t: &T) {
    // because T may not be a known size a compile time
    // we must use a pointer of some kind
}

fn bar() -> ! {
    None
}
type Foo = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Foo) {
    (*f)()
}

fn returns_long_type() -> Foo {
    Box::new(|| println!("Hey!"))
}
fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("Hello, world!");
    println!("x + y = {}", x + y);

    // using synonyms to deal with long-winded types
    let f: Foo = Box::new(|| println!("hi"));
    takes_long_type(f);
    let f2 = returns_long_type();
    takes_long_type(f2);
}
