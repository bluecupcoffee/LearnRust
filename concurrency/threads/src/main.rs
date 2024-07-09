use std::error::Error;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi! This is number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi! This is number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1,2,3];

    let handle = thread::spawn( move || {
        println!("Here's a vector {v:?}");
        v
    });


    let v = handle.join().unwrap_or_else(|e| {
        eprintln!("{e:?}");
        process::exit(1);
    });

    drop(v);
}
