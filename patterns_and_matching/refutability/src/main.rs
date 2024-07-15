fn main() {
    println!("Hello, world!");
    let o = Some(Box::new(1));
    // leads to compiler error because None is not handled
    // let Some(x) = o;

    // OK
    if let Some(x) = o.as_ref() {
        println!("{x}");
    }

    println!("{o:?}");
}
