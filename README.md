# exitcode
System exit code constants as defined by [sysexits.h](https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html)

# Installing
```
$ cargo install exitcode
```
# Example
```
extern crate exitcode;

::std::process::exit(exitcode::OK);
```
