//! Rust program to print hello world.
//!
//! keywords: hello world, program, main, function, print
//!
//! This program, on invocation, prints the bytes "Hello, world!" to the standard output
//! buffer (also known as the terminal, the console, the CLI, the tty or the teletypewriter)
//! and flushes it, and then exits. The program may panic if writing to the standard
//! output fails.
//! more on panics later.
//!
//! Usage (in the terminal):
//!
//! # using Cargo, Rust's package manager
//!     cargo run
//! # precompiled program
//!     # windows
//!     hello_world.exe
//!     # linux
//!     ./hello_world
//!
//! Voxell Paladynee 2025

// +-------------------------------------------------+
// | Rust Editor.exe                           _ o x |
// +----|--------------------------------------------+
// | 32 |                                            |
// | 33 | fn main() {                                |
// | 31 | --------- `main` is a special function!    |
// +----|--------------------------------------------+
//
// the entry point of our program, named `main`. the standard library will automatically call this function
// upon program invocation, so it is a special function. it can return any value that implements `Terminate`.
// this function returns nothing, and nothing is one such type.
// more on `impl`ementations, `functions` and the `nothing` type later.
fn main() {
    // we're inside the `main` function, and we encounter our first scope with a non-'static lifetime.
    // anything declared in this function, like variables, do not "live" after main exits.
    // more on `lifetimes`, `'static` and `scopes` later.

    // the standard library and user-written types that have `Drop` implementations
    // may do some cleanup after the main function exits, after then which the program
    // terminates automatically.
    // more on `Drop` later.

    //      the &str type
    //     +--------+--------+
    //     | 0x0123 |     13 |
    //     +--------+--------+
    //          |
    //          v
    //     +--------+--------+--------+--------+--------+--------+
    //     |    'H' |    'e' |    'l' |    'l' |    'o' |    ' ' | ...
    //     +--------+--------+--------+--------+--------+--------+
    //      hello.exe contents
    
    // here we define a `string literal` with an inferred size. the string literal actually
    // has 'static lifetime, because it is actually a pointer to the to-be-compiled binary program,
    // which exists regardless of whether we're inside `main` or not. therefore,
    // Rust allows us to make values with 'static lifetimes.

    // the `&str` type holds a compile-time guaranteed UTF-8 valid sequence of bytes,
    // therefore it is always safe for us to write this value to the standard output.
    let string: &'static str = "Hello, world!";

    // here we invoke the `println!` macro, which is included with the Rust standard library
    // (std) implicit prelude. It takes a format string, and a variable amount of arguments
    // to print some values to the standard output, followed by a new line (`\n`) byte.
    // the println! function may panic if writing or flushing standard output fails.

    // here, the `string` value implements the `Display` trait from the `std::fmt` module,
    // which when implemented on any value denotes how the value should be formatted
    // to a target `String` for end-user viewing purposes. This trait is what `println!`
    // actually looks for when it's given a value as an argument. For a simple `&str` type,
    // this implementation directly writes the bytes of the underlying string to the target
    // `String` buffer.
    println!("{}", string);

    // normally, `string` would be dropped after this scope ends at the next close bracket,
    // but because string slices don't implement `Drop` (don't have a "drop glue"), no additional
    // action will be taken by the compiler to get rid of that value, and the stack frame of the
    // function is reclaimed immediately after the function exits, which the standard library will
    // use to do some cleanup as an implementation detail.
}
