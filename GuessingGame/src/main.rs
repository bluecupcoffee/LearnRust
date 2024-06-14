use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret: u8 = rand::thread_rng().gen_range(0..255);
    let mut tries: u8 = 0;

    loop {
        println!("Guess the number! (between 0 and 256 exclusive)");
        println!("Enter your guess: ");
        let mut guess_string = String::new();

        io::stdin().read_line(&mut guess_string)
            .expect("Failed to read line");

        let guess:u8 = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };


        println!("You guessed: {}", guess);
        tries += 1;
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too high")
        }
    }
    println!("Won in {} tries!", tries);
}
