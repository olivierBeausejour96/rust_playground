#[cfg(test)]
mod tests {
    use super::*;
    mod config_tests {
        use super::*;
        #[test]
        fn config_constructor() {
            let args = vec![String::from("executable_path"), String::from("query"), String::from("file_path")];
    
            let c = Config::new(&args).unwrap();
    
            assert_eq!(c.query, args[1]);
            assert_eq!(c.path, args[2]);
        }

        #[test]
        #[should_panic]
        fn config_constructor_error() {
            let args = vec![String::from("executable_path"), String::from("query")];

            Config::new(&args).unwrap();
        }
    }

    mod search_tests {
        use super::*;
        #[test]
        fn one_result() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
pick three.";

            assert_eq!(
                vec!["safe, fast, productive."], 
                search(query, contents)
                );
        }
    }
}

use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    path:  String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters provided!");
        }
        
        let query = args[1].clone();
        let path  = args[2].clone();

        Ok(Config { query, path })
    }
}

pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    
    Config::new(&args)

}

use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(&config.path)?;

    println!("{}", content);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();

    for l in contents.lines() {
        if l.contains(query) {
            ret.push(l);
        }
    }

    ret

}





















