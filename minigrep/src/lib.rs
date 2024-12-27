use std::{error::Error, fs, env};

pub struct Config{
    file_path: String,
    query: String,
    ignore_case: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config{file_path, query, ignore_case}
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Should be at least 3 args needed");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{
            file_path, 
            query, 
            ignore_case}
        )
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>>{
    //read a file
       let content = fs::read_to_string(conf.file_path).expect("Have been able to read file");
       let results = if conf.ignore_case {
            search_case_insensitive(&conf.query, &content)
       }else {
            search(&conf.query, &content)
       };

       for line in results {
           println!("{line}");
       }
   
       Ok(())
}

pub fn search <'a> (query: &str, contents: &'a str) ->Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            result.push(line.trim());
        }
    }
    return result;
}

pub fn search_case_insensitive <'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let lq = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&lq){
            result.push(line.trim());
        }
    }
    return result;
    
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn search_sensitive(){
        let query = "duct";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
    #[test]
    fn search_insensitive(){
        let query = "rUsT";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.
        ";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}