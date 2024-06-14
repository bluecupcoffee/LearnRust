use std::io;
use std::cmp::Ordering;
use std::process::exit;

fn main() {
    println!("Enter temp in F°:");
    let mut f = String::new();
    io::stdin().read_line(&mut f);
    let f = match f.trim().parse::<f32>() {
        Ok(num) => num,
        Err(E) => {
            println!("Errored out reading value");
            exit(1);
        }
    };
    println!("{}F in C is {}", f, fahrenheit_to_celsius(f));
    println!("16th fibonacci number is {}", nth_fibonacci_number(16, 16));
    n_days_of_christmas();
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    return (f - 32.0f32) * (5.0f32/9.0f32);
}

fn nth_fibonacci_number(ith: u64, nth: u64) -> u64 {
    // Fn = Fn-1 + Fn-2
    if ith == 0 {
        return 0;
    }
    if ith == 1 {
        return 1;
    }
    if ith == 2 {
        return 1;
    }

    return nth_fibonacci_number(ith - 1, nth) + nth_fibonacci_number(ith - 2, nth);
}

fn n_days_of_christmas() {
    for n in (1..13) {
        println!("Something something on the {} day of Christmas", n);
    }
}
