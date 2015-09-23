use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::{ BufRead, BufReader };

pub fn main() {
    let mut argv = env::args();
    let file_name: String = match argv.nth(1) {
        Some(s) => s,
        None    => {
            println!("Expected filename");
            exit(1)
        }
    };
    
    let path = Path::new(&file_name);
    
    //Read the file as a stream of lines of utf8
    let file = File::open(&path).unwrap();
    let read = BufReader::new(file);
    
    for line in read.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}