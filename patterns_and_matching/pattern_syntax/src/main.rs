fn main() {
    println!("Hello, world!");

    // matching literals

    let x = 1;
    match x {
        1 => println!("one"),
        _ => println!("anything else")
    }

    // matching named variables

    let x = Some(5);
    let y = 10;

    // match a specific value
    // match any value inside a Some
    // default case
    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("Matched y = {y}"),
        _ => println!("Default case, x = {x:?}")
    }

    println!("x = {x:?}, y = {y}");

    // multiple patterns and ranges
    let x = 10;
    match x {
        1 | 2 => println!("one or two"),
        1..=11 => println!("one through eleven"), // ..= is inclusive range
        _ => println!("something else"),
    }

    // destructuring to break apart values
    let p =  Point {
        x: 1,
        y: 100
    };

    let Point { x: a, y: b} = p;
    assert_eq!(a, 1);
    assert_eq!(b, 100);

    let p = Point { x: 0, y: 7};

    match p {
        Point {x, y: 0} => println!("on the x axis at {x}"),
        Point{x:0, y} => println!("on the y axis at {y}"),
        Point{x, y} => {
            println!("on neither axis: {x}, {y}");
        }
    }

    // destructuring
    let mut v: Vec<Message> = vec![];
    v.push(Message::ChangeColor(0, 160, 255));
    v.push(Message::Quit);
    v.push(Message::Move{x: 5, y: 5});
    v.push(Message::Write(String::from("Hi there")));

    while let Some(msg) = v.pop() {

        match msg {
            Message::Quit => { println!("Quit!") },
            Message::Move {x, y} => {
                println!("Move in the x direction {x} and in the y direction {y}")
            },
            Message::Write(text) => println!("{text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, blue {b}");
            }

        }
    }

    // destructuring nested structs and enums
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => ()
    }

    // destructuring structs and tuples
    let ((feet, inches) , Point{ x, y}) = ((3, 10), Point{x: -5, y: 10 });
    println!("{feet}, {inches}");
    println!("{x}, {y}");

    // ignoring values in a pattern
    foo(3,4);

    // ignoring parts of a value with a nest _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {setting_value = new_setting_value;}
    }

    println!("setting is {setting_value:?}");

    // ignoring an unused variable
    let _zzz = 10;

    // the _ ignores the value and ownership is not moved from s to _
    let s = Some(String::from("test"));
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    // ignore the remaining parts of a value with ..
    let origin = Point2 { x: 0, y: 0, z: 0};

    match origin{
        Point2 {x, ..} => println!("x is {x}"),
    }

    // .. expands to as many numbers as it needs
    let n = (1,2,3,4,5);
    match n {
        (first,.., fourth, last) => {
            println!("Some numbers {first}, {fourth}, {last}");
        }
    }
    // cannot be ambiguous
    // this will cause an error
    // match n {
    //     (.., mid, ..) => {
    //         println!("{mid}");
    //     }
    // }

    // extra conditionals with match guards
    // the condition can use vars created in the pattern
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => ()
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("default case, x = {x:?}")
    }

    println!("at the end x = {x:?}, y = {y}");

    // @ bindings
    // the @ operator creates a variable that holds a value at
    // the same time as we test the value for a pattern match

    let msg = Message3::Hello{id: 5};

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7
        } => { println!("id in range: {id_variable}")},
        Message3::Hello {id: 10..=12} => {
            println!("Found id in another range");
        },
        Message3::Hello{id} => println!("Found some other id: {id}")
    }

}

fn foo(_: i32, y:i32) {
    println!("This code only uses the 2nd parameter");
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

enum Message3 {
    Hello {id: i32}
}
enum Message2 {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(Color)
}

struct Point {
    x: i32,
    y: i32
}
struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}