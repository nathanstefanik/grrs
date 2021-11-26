use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    
    let results = if config.case_sensitive {
        search(&config.pattern, &contents)
    } else {
        search_case_insensitive(&config.pattern, &contents)
    }; 

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config  {
    pub pattern: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let pattern = args[1].clone();
        let path = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { pattern, path, case_sensitive })
    }
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&pattern) {
            results.push(line);
        }
    }
    
    results
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let pattern = pattern.to_lowercase();
    for line in contents.lines() { 
        if line.to_lowercase().contains(&pattern) {
            results.push(line);
        }
    }
    
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let path = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(pattern, path));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(pattern, contents)
        );
    }
}