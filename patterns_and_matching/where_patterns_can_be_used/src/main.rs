fn main() {
    println!("Hello, world!");
    let x = 1;
    match x {
        1 => println!("{}", x),
        2 => println!("{}", x * 2),
        3 => println!("{}", x * 3),
        _ => println!("{}", x * x)
    }

    let so = Some::<String>(String::from("hi there"));
    match so {
        Some(s) => println!("{s}"),
        None => println!("nothin")
    }

    let so = Some::<String>(String::from("Yet another great find!"));
    if let Some(s) = so {
        println!("Looks like there's a string here: {}", s);
    } else {
        println!("Looks like there's no string here");
    }


    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec![ 'a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let point = (3,2);

    print_coords(&point);

}


fn print_coords(&(x, y): &(i32, i32)) {
    println!("current location: ({x}, {y})");
}