extern crate exitcode;

use std::process;
use std::io::{Write, stderr};

pub fn parse_int_or_return_error_exitcode(s: &str) -> Result<i32, exitcode::ExitCode> {
    match s.parse::<i32>() {
        Ok(i) => Ok(i),
        Err(_) => Err(exitcode::USAGE)
    }
}

pub fn main() {
    match parse_int_or_return_error_exitcode("foo") {
        Ok(i) => {
            println!("Parsed: {}", i);
            process::exit(exitcode::OK);
        },
        Err(code) => {
            writeln!(stderr(), "Parse error.  Exiting with code: {}", code).unwrap();
            process::exit(code);
        }
    }
}
