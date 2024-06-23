use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(&self.filename)?;
        println!("With text:\n{contents}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
