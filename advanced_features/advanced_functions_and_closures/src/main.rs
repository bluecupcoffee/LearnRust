// function pointers

// fn type is called a function pointer
fn add_one(x: u32) -> u32 {
    x + 1
}

fn do_twice(f: fn(u32) -> u32, arg: u32) -> u32 {
    f(arg) + f(arg)
}

fn do_thing(f: fn() -> ()) -> fn() {
    f
}

fn f1(input: u32) {
    println!("print line 1");
    println!("print line 2");
    println!("input is {input}");
}

fn f2(input: u32) {
    let i = (input * 2) + 10;
    println!("(input * 2) + 10 = {i}");
    println!("input used was {input}");
}
#[derive(Debug)]
struct MyCollection(Vec<()>);
impl MyCollection {
    fn new() -> MyCollection {
        MyCollection(Vec::new())
    }

    fn add(&mut self, elem: ()) {
        self.0.push(elem);
    }
}

impl FromIterator<()> for MyCollection {
    fn from_iter<T: IntoIterator<Item=()>>(iter: T) -> Self {
        let mut c = MyCollection::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop
}

// returning closures
// closures are represented by traits, so you can't return a closure directly
// instead you can use a concrete type that implements
// the trait as a return value of the function
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn main() {
    println!("Hello, world!");
    let answer = do_twice(add_one, 5);
    println!("add_one done twice with 5: {}", answer);
    let v = vec![f1, f2];
    for (i,f) in v.iter().enumerate() {
        f(i as u32);
    }

    let list_of_numbers: Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10];
    let _: MyCollection = list_of_numbers.iter().map(|i| f2(*i)).collect();

    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("\n{}\n{:?}\n{}\n", "-".repeat(80), list_of_strings, "-".repeat(80));
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("\n{}\n{:#?}\n{}\n", "-".repeat(80), list_of_statuses, "-".repeat(80));

    let add_1 = returns_closure();
    println!("{}", add_1(10));

}
