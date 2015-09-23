use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::io;
use std::num;
use std::convert::From;

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
        Ok(numbers) => {
            //Success! Have a vec of integers
            for n in numbers.iter() {
                println!("{}", n);
            }
        },
        Err(e) => {
            //Error! Something went wrong
            match e {
                ReadError::Io(err)    => println!("Error reading file: {}", err),
                ReadError::Parse(err) => println!("Error parsing file: {}", err)
            }
            exit(1)   
        }
    }
}

// An enumeration of the possible errors we'll encounter
enum ReadError {
    Io(io::Error),
    Parse(num::ParseIntError)
}

fn read_file(path: &Path) -> Result<Vec<u64>, ReadError> {
    let file = try!(File::open(&path));
    let read = BufReader::new(file);
    let mut numbers = Vec::new();
    for line in read.lines() {
        let line = try!(line);
        let n    = try!(line.trim().parse());
        numbers.push(n);
    }
    Ok(numbers)
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> ReadError {
        ReadError::Io(e)
    }
}

impl From<num::ParseIntError> for ReadError {
    fn from(e: num::ParseIntError) -> ReadError {
        ReadError::Parse(e)
    }
}