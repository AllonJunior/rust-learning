use std::f32::consts::E;
use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn open_file_with_match(filename: &str) {
    let greeting_file_result = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("Successfully opened!");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found,will create a new one!");
                match File::create(filename) {
                    Ok(fc) => {
                        println!("File successfully created!");
                        fc
                    }
                    Err(e) => panic!(""),
                }
            }
            other_errors => {
                panic!("");
            }
        },
    };

}

fn open_file_unwrap_error(filename: &str) {
    let greeting_file = File::open(filename).unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error|{
                panic!("Problem when creating file {:?}", error);
            })
        } else {
            panic!("Problem when open file {:?}", error);
        }
    });
}

fn read_username_from_file(filename: String) -> Result<String, io::Error> {
    let open_file_result = File::open(filename);
    let mut open_file = match open_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match open_file.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut(filename: String) ->Result<String, io::Error>{

    // version 1
    // let mut username_file = File::open(filename)?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    
    // // version 2
    // let mut username = String::new();
    // File::open(filename)?.read_to_string(&mut username)?;

    // Ok(username)

    // version 3
    fs::read_to_string(filename)
}
fn main() {
    // println!("Hello, world!");
    // panic!("crash and burn");
    // let v = vec![1,2,3];
    // v[10];
    // open_file_with_match("greeting.txt");
    // open_file_unwrap_error("greeting_unwrap.txt");
    match read_username_from_file(String::from("hello.txt")){
        Ok(username) => println!("{username}"),
        Err(e) => println!("Problem when reading {:?}", e),
    };

    match read_username_from_file_shortcut(String::from("hello_shortcut.txt")){
        Ok(username) => println!("{username}"),
        Err(e) => println!("Problem when reading hello_shortcut {:?}", e),
    };
}

