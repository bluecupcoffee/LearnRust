use std::sync::mpsc;
use std::thread;
use std::process;
use std::time::Duration;

fn main() {
    // println!("Hello, world!");
    // let (tx, rx) = mpsc::channel();
    //
    // let t1 = thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    //
    // let received = rx.recv();
    // match received {
    //     Ok(o) => {
    //         println!("Received {o}");
    //     },
    //     Err(e) => {
    //         eprintln!("Error! {e}");
    //         process::exit(1);
    //     }
    // }
    //
    // t1.join().unwrap();
    // let (tx, rx) = mpsc::channel();
    //
    // let t2 = thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread")
    //     ];
    //
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    //
    // });
    //
    // for r in rx {
    //     println!("Got: {r}");
    // }
    //
    let (tx, rx) = mpsc::channel();
    let t1 = tx.clone();
    //
    // println!("Break before more threads\n\n\n\n");
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for v in vals {
            let s = String::from(v.to_string() + " from T1");
            t1.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for v in vals {
            let s = String::from(v.to_string() + " from T2" );
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        println!("{r}");
    }
}
