use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::io;

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

// try! macro style
// See: http://doc.rust-lang.org/stable/std/macro.try!.html for the full macro

fn read_file(path: &Path) -> Result<Vec<String>, io::Error> {
    let file = try!(File::open(&path));
    let read = BufReader::new(file);
    let mut lines = Vec::new();
    for line in read.lines() {
        lines.push(try!(line));
    }
    Ok(lines)
}