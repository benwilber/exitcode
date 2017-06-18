[![ghit.me](https://ghit.me/badge.svg?repo=benwilber/exitcode)](https://ghit.me/repo/benwilber/exitcode) [![Build Status](https://travis-ci.org/benwilber/exitcode.svg?branch=master)](https://travis-ci.org/benwilber/exitcode) [![Crates.io](https://img.shields.io/crates/v/exitcode.svg)](https://crates.io/crates/exitcode)

# exitcode
System exit code constants as defined by [sysexits.h](https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html)

Documentation is available [here](https://docs.rs/exitcode)

# Installing from [crates.io](https://crates.io/crates/exitcode)
```
[dependencies]
exitcode = "1.1.2"
```

# Example
```rust
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
```
