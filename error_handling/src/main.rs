use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

//     let _v = vec![1,2,3];
//     let error_int = _v[99];

    let _f = File::open("example.txt");
    let _f = match _f {
        Ok(f) => {
            println!("Opened file!");
            f
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("example.txt") {
                Ok(file) => {
                    println!("Created file!");
                    file
                },
                Err(e) => panic!("Error creating the file:\n{:#?}", e)
            },
            other_error => panic!("Problem opening the file:\n{:#?}", other_error)
        }
    };

    // this breaks things because it panics uncomment when done
    // let _f2 = File::open("ex2.txt").unwrap();

    // this also breaks things because it panics. uncomment when done
    // let _f3 = File::open("ex3.txt")
    //     .expect("Couldn't open ex3.txt");

    println!("{:#?}", _f);
    println!("Error#1:\n{:#?}", read_file());
    println!("Error#2:\n{:#?}", read_file_shorter());
    println!("Error#3:\n{:#?}", read_file_even_shorter());
}

fn read_file() -> Result<String, std::io::Error> {
    let mut f = File::open("text.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_shorter() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("text.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

//this one doesn't error. just checking out the implementation of a concise function
fn read_file_even_shorter() -> Result<String, std::io::Error> {
    std::fs::read_to_string("text.txt")
}

struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if (value < 1 || value > 100) {
            panic!("Value should be between 1(inclusive) and 100(inclusive)");
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
