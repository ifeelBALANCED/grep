use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let (query, file_path) = match (args.next(), args.next()) {
            (Some(query), Some(file_path)) => (query, file_path),
            _ => return Err("not enough arguments"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_lines = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in search_lines {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the case-sensitive search function correctly finds a match
    #[test]
    fn case_sensitive() {
        // Setup
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        // Action
        let result = search_case_sensitive(query, contents);

        // Assertion
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    /// Test that the case-insensitive search function correctly finds a match
    #[test]
    fn case_insensitive() {
        // Setup
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        // Action
        let result = search_case_insensitive(query, contents);

        // Assertion
        assert_eq!(vec!["Rust:", "Trust me."], result);
    }
}