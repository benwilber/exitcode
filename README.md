[![ghit.me](https://ghit.me/badge.svg?repo=benwilber/exitcode)](https://ghit.me/repo/benwilber/exitcode) [![Build Status](https://travis-ci.org/benwilber/exitcode.svg?branch=master)](https://travis-ci.org/benwilber/exitcode)

# exitcode
System exit code constants as defined by [sysexits.h](https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html)

Documentation is available [here](https://docs.rs/exitcode)

# Installing from [crates.io](https://crates.io/crates/exitcode)
```
[dependencies]
exitcode = "1.1.0"
```
# Example
```
extern crate exitcode;

::std::process::exit(exitcode::OK);
```
