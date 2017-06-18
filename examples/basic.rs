extern crate exitcode;

use std::process;

pub fn parse_int_or_return_error_exitcode(s: String) -> Result<i32, exitcode::ExitCode> {
    match s.parse::<i32>() {
        Ok(i) => Ok(i),
        Err(_) => Err(exitcode::USAGE)
    }

}

pub fn main() {

    match parse_int_or_return_error_exitcode("123".to_string()) {
        Ok(i) => println!("Parsed: {}", i),
        Err(code) => {
            println!("Parse error.  Exiting with code: {}", code);
            process::exit(code);
        }
    }

    match parse_int_or_return_error_exitcode("foo".to_string()) {
        Ok(i) => println!("Parsed: {}", i),
        Err(code) => {
            println!("Parse error.  Exiting with code: {}", code);
            process::exit(code);
        }
    }

    println!("Exiting with code: {}", exitcode::OK);
    process::exit(exitcode::OK);

}
