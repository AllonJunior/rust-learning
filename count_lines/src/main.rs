use std::{env, fs};
use std::io::{self, BufRead};
use std::path::Path;

fn count_lines_in_file(file_path: &Path) -> io::Result<usize> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}

fn count_lines_in_directory(dir_path: &Path) -> io::Result<usize>{
    let mut total_lines = 0;

    for entry in fs::read_dir(dir_path)?{
        let entry = entry?;
        let path = entry.path();

        if path.is_file(){
            if let Some(extension) = path.extension(){
                if extension == "md" || extension == "plantuml"{
                    total_lines += count_lines_in_file(&path)?;
                }
            }
        } else if path.is_dir(){
            total_lines += count_lines_in_directory(&path)?;
        }
    }

    Ok(total_lines)
}

fn main() -> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory path>", args[0]);
        std::process::exit(1);
    }

    let dir_path = Path::new(&args[1]);
    let total_lines = count_lines_in_directory(&dir_path)?;
    println!("In directory {}, total lines of all .md and .plantuml files are: {}", dir_path.display(), total_lines);
    Ok(())
}
