fn main() {
    println!("Hello, world!");
    let guitar: [GuitarString; 6] = [
        GuitarString::E(String::from("hi")),
        GuitarString::B,
        GuitarString::G,
        GuitarString::D,
        GuitarString::A,
        GuitarString::E(String::from("lo")),
    ];

    for g_string in guitar.iter() {
        g_string.print_string();
    }
}
#[derive(Debug)]
enum GuitarString {
    E(String),
    B,
    G,
    D,
    A,
}

impl GuitarString {
    fn print_string(&self) {
        if let GuitarString::E(gs) = self {
            println!("E {}", gs);
        } else {
            println!("{:?}", self);
        }
    }
}
