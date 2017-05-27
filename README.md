# exitcode
System exit code constants as defined by [sysexits.h](https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html)

Documentation is available [here](https://docs.rs/exitcode)

# Installing from [crates.io](https://crates.io/crates/exitcode)
```
[dependencies]
exitcode = "1.0.1"
```
# Example
```
extern crate exitcode;

::std::process::exit(exitcode::OK);
```
