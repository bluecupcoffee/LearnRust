use std::sync::{Mutex, Arc};
use std::thread;
use std::process;

fn main() {
    println!("Hello, world!");
    let m = Mutex::new(5);
    {
        match m.lock() {
            Ok(mut o) => {
                *o = 6;
            },
            Err(perr) => {
                let data = perr.get_ref();
                println!("Recovered: {data}");
            }
        }
    }

    println!("m = {m:?}");


    let counter = Arc::new(Mutex::new(0u8));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            match counter.lock() {
                Ok(mut o) => { *o += 1; },
                Err(e) => {
                    eprintln!("Error getting mutex lock {}", e.to_string());
                    process::exit(1);
                }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        match h.join() {
            Ok(_o) => (),
            Err(e) => {
                eprintln!("Error joining handles {e:?}");
                process::exit(1);
            }
        }
    }

    match counter.lock() {
        Ok(o) => {
            println!("Result: {}", *o);
        },
        Err(e) => {
            eprintln!("{e:#?}");
            process::exit(1);
        }
    };

}
