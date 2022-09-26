use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath, })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filepath)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

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