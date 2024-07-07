use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi! This is number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi! This is number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
}
