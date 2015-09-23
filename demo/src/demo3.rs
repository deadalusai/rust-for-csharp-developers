use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::{ BufRead, BufReader };

pub fn main() {
    let mut argv = env::args();
    let file_name: String = match argv.nth(1) {
        Some(s) => s,
        None => {
            println!("Expected filename");
            exit(1)
        }
    };
    
    let path = Path::new(&file_name);
    
    match read_file(&path) {
        Ok(lines) => {
            //Success! Have a vec of strings
            for line in lines.iter() {
                println!("{}", line);
            }
        },
        Err(message) => {
            //Error! Something went wrong
            println!("An error occured: {}", message);
            exit(1)
        }
    }
}

fn read_file(path: &Path) -> Result<Vec<String>, &str> {
    
    //Read the file as a stream of lines of utf8
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Err("Could not open file")  
    };
    let read = BufReader::new(file);
    
    let mut lines = Vec::new();
    
    for line in read.lines() {
        let line = match line {
            Ok(s) => s,
            Err(_) => return Err("An error occured while reading a line")
        };
        lines.push(line);
    }
    
    Ok(lines)
}