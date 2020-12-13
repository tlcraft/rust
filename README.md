# README

We're reading through the Rust Programming Language and working on some supplemental exercises as part of a book club.

TRPL: [https://doc.rust-lang.org/book/title-page.html](https://doc.rust-lang.org/book/title-page.html)

## Example of running on Windows

$ rustc hello_world.rs
$ hello_world

## Cargo commands

cargo build --This compiles the program and creates an executable file. This also generates a cargo.lock file to track your dependency versions.

cargo run  --This command will compile the program (if changes are detected) and automatically run the executable.

cargo check  --This verifies that the program can compile but won't produce the executable which saves on compile time.
