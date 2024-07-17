use std::fmt;
use std::ops::Add;
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
trait OutlinePrint :fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point3D {}
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

struct Human;

trait Pilot {
    fn fly(&self);
}
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms*");
    }
}
trait Wizard {
    fn fly(&self);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
struct F {
    curr: u32,
    next: u32
}

impl Iterator for F {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn f() -> F {
    F {curr: 0, next: 1}
}

fn main() {
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in f().take(4) {
        println!("> {}", i);
    }

    // performs next() but does not execute the inside of code.
    // like hitting continue from within a block if below n in skip(n)
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in f().skip(4).take(4) {
        println!("> {}", i);
    }

    // rust doesn't let you create your own operators or overload
    // arbitrary operators. Basically limited to operators in std::ops
    // by implementing traits associated with the operator
    assert_eq!(
        Point3D { x: 1, y: 2, z: 3} + Point3D { x: 4, y: 5, z: 6},
        Point3D { x: 5, y: 7, z: 9}
    );

    let length_in_mm = Millimeters(1000);
    let length_in_m = Meters(4);
    assert_eq!(length_in_mm + length_in_m, Millimeters(5000));
    // you can full qualify with traits to reduce ambiguity

    let h = Human;
    h.fly();
    Wizard::fly(&h);
    Pilot::fly(&h);

    println!("A baby dog is a {}", <Dog as Animal>::baby_name());
    println!("A baby dog named {}", Dog::baby_name());

    // using supertraits to require a trait's functionality
    // within another trait
    let p1 = Point3D {
        x: 0,
        y: 1,
        z: 2
    };
    p1.outline_print();

    // getting around the orphan rule with newtype pattern
    // can create a new type in a tuple struct to make a wrapper
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
    //let v = vec![String::from("hello"), String::from("world")];
    //println!("v = {v}");
}

