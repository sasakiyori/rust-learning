use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip first argument: binary name
        args.next();

        // transfer the ownerships!
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("missing query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("missing file path"),
        };

        Ok(Config { query, file_path })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file_path)?;

    for line in search(&cfg.query, &contents) {
        println!("{line}");
    }

    println!("{contents}");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
