use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filepath, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }
    else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";
        assert_eq!(
            vec![
                "Rust:",
                "Trust me."
            ],
            search_case_insensitive(query, content)
        );
    }

    #[test]
    #[should_panic]
    fn config_arg0() {
        let args: Vec<String> = vec![];
        Config::new(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn config_arg1() {
        let self_str = String::from("lib.rs");
        let args: Vec<String> = vec![self_str];
        Config::new(&args).unwrap();
    }

    #[test]
    #[should_panic]
    fn config_arg2() {
        let self_str = String::from("lib.rs");
        let word_str = String::from("word");
        let args: Vec<String> = vec![self_str, word_str];
        Config::new(&args).unwrap();
    }

    #[test]
    fn config_arg3() {
        let self_str = String::from("lib.rs");
        let word_str = String::from("word");
        let file_str = String::from("tests/test.txt");
        let args: Vec<String> = vec![self_str, word_str, file_str];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, args[1]);
        assert_eq!(config.filepath, args[2]);
    }

    #[test]
    fn config_arg4() {
        let self_str = String::from("lib.rs");
        let word_str = String::from("word");
        let file_str = String::from("tests/test.txt");
        let append_str = String::from("append");
        let args: Vec<String> = vec![self_str, word_str, file_str, append_str];

        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, args[1]);
        assert_eq!(config.filepath, args[2]);
    }
}