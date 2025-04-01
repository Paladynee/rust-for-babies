// thanks to the Cargo toolchain, compiling rust code is a breeze.
// in order to write some code, test it, and execute it, the workflow looks like follows:
// (please follow along)

// 0. make sure you have Rust installed.
// https://www.rust-lang.org/tools/install

// 1. make a new project directory
//
// in bash, or windows terminal, or in your favourite file explorer,
// make a new directory in which your project will live. if you haven't opened
// a terminal already, open a terminal and navigate to the new directory.

// 2. invoke `cargo` to make a new minimal project for you
//
// run `cargo init` to make a new project in the current directory, with
// the current directory's name as the "crate" name. crates are the names
// given to projects in Rust, and they encapsulate the source files
// for the project along with `Cargo.toml` and other files telling
// cargo how to build things.

// 3. open your favourite editor in src/main.rs
//
// it's time to hop on your editor, and do some programming. mash your keys,
// write your logic, and come back. for now, you can copy the hello world
// main function, and replace everything in `main.rs` with it.

// 4. compile and run the code using cargo
//
// with cargo, compilation is a breeze. in the project root (not in the src directory),
// run `cargo run` to compile and run your program. you can run `cargo run --release`
// to make the program compile slower, but run faster in runtime. this is called a
// "release build", and they employ compiler optimizations to make your programs run
// blazingly fast.

// 5. test your code using cargo
//
// in Rust, any function denoted with `#[test]` above it
// will be run as a test when cargo is invoked with `cargo test`.
// for a simple demonstration, try writing a new function and
// add `#[test]` to the line above it. these functions are called
// unit tests, and are mainly used to test single units of abstraction.
// for example:
// ```rs
// #[test]
// fn my_test() {
//     assert!(false != true);
// }
// ```
// you can now run `cargo test` to run your unit tests, and make sure your
// program works as intended.

// 6. bonus step: making a library crate
//
// start from all over, but in step 2, run `cargo init --lib` instead.
// you'll have a lib.rs file in the src directory. these crates
// have the library type, and don't have an entry point, as such, they
// don't provide directly executable programs. you can't use `cargo run`
// to run them. instead, you have to use `cargo test` to run your unit tests.

fn main() {
    println!("Hello, world!");
}
