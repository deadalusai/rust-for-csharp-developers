use std::env;
use std::process::exit;

pub fn main() {
    let mut argv = env::args();
    let arg: String = match argv.nth(1) {
        Some(s) => s, // Success - resolve to the string value
        None    => {
            //Exit with an error
            println!("Not enough arguments");
            exit(1)
        }
    };
    let n: i32 = match arg.parse() {
        Ok(i)  => i, // Success - resolve to the i32 value
        Err(e) => {
            //Exit with an error
            println!("Could not parse argument: {:?}", &e);
            exit(1)
        }
    };
    println!("{}", n);
}