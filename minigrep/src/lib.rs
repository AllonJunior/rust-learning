use std::{error::Error, fs};

pub struct Config{
    file_path: String,
    query: String,
}
impl Config {
    pub fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config{file_path, query}
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("Should be at least 3 args needed");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{file_path, query})
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>>{
    //read a file
       let content = fs::read_to_string(conf.file_path).expect("Have been able to read file");
       println!("Poem with content:\n{content}");
   
       Ok(())
}