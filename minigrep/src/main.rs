//enable to read command args
use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::build(&args).unwrap_or_else(|err|
    {
        eprintln!("Problem occured: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(conf){
        eprintln!("Error occured: {e}");
        process::exit(1);
    }
   
}
