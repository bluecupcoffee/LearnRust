use rand::Rng;
use rand::distributions::uniform::SampleUniform;
use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;
use std::fmt;
use std::error::Error;
use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
    let mut g1: Guess<u32> = Guess::new(0, 512);
    g1.play();
}
#[derive(Debug)]

struct Guess<T> {
    guess_min: T,
    guess_max: T,
    guess_tgt: T,
    last_guess: Option<T>,


}

#[derive(Debug)]
struct GuessError {
    err: String,
    guess_input: Option<String>,
}

impl GuessError {
    fn new(message: String, input: Option<String>) -> GuessError {
        GuessError { 
            err: message.to_string(),
            guess_input: input
        }
    }
}


impl fmt::Display for GuessError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.err)
    }
}

impl Error for GuessError {
    fn description(&self) -> &str {
        &self.err
    }
}

impl<T: Ord + SampleUniform + Copy + Display + FromStr> fmt::Display for Guess<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Min: {}\nMax: {}\nTarget: {}\nLast guess: {}", 
                self.guess_min, 
                self.guess_max, 
                self.guess_tgt,
                match self.last_guess{ Some(t) => t.to_string(), None => "none".to_string()})
    }
}


impl<T: Ord + SampleUniform + Copy + Display + FromStr> Guess<T> {
    fn new(min: T, max: T) -> Guess<T> {
        Guess {
            guess_min: min,
            guess_max: max,
            guess_tgt: Self::get_rnd_val(&min, &max),
            last_guess: None
        }
    }

    fn get_rnd_val(lower_bound: &T, upper_bound: &T) -> T {
        let secret: T = rand::thread_rng().gen_range(*lower_bound..*upper_bound);
        secret
    }

    fn cmp_guess<'a>(&'a mut self, guess: &T) -> (&'a Option<T>, bool) {
        match guess.cmp(&self.guess_tgt) {
            Ordering::Less => {
                println!("too low!");
                self.last_guess = Some(*guess);
                return (&self.last_guess, true)
            },
            Ordering::Equal => {
                println!("you win!");
                self.last_guess = Some(*guess);
                return (&self.last_guess, false);
            },
            Ordering::Greater => {
                println!("too high!");
                self.last_guess = Some(*guess);
                return (&self.last_guess, true)
            }
        }
    }

    fn get_guess(&self) -> (bool, Result<T, GuessError>) {
        let mut exit = false;
        println!("Take a guess between {} and {}", self.guess_min, self.guess_max);
        match self.last_guess {
            Some(n) => println!("Your last guess was {}", n),
            None => println!("This is your first guess!")
        };
        let mut guess_string = String::new();
        let _ = std::io::stdin().read_line(&mut guess_string)
                    .expect("Couldn't read user input");
        match guess_string.as_str() {
            "exit" => exit = true,
            _ => {}
        }


        (exit, match guess_string.trim().parse::<T>() {
            Ok(g) => Ok(g),
            Err(_) => Err(GuessError::new(String::from("Error parsing string"), Some(guess_string)))
        })
    }

    fn play(&mut self) {
        let mut cont = true;
        while cont {
            let guess = self.get_guess();
            if guess.0 { break; }
            let guess = match guess.1 {
                Ok(g) => g,
                Err(e) => {
                    println!("{:#?}", e);
                    continue;
                }
            };

            let res = self.cmp_guess(&guess);
            match res.0 {
                Some(g) => println!("You guessed: {}", g),
                None => {}
            }

            cont = res.1;

            


        }
        
    }
}