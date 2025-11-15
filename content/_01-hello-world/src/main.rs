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
//! [terminal, console, CLI, tty]: https://en.wikipedia.org/wiki/Command-line_interface
//! [Rust]: https://github.com/rust-lang/rust
//! [Cargo]: https://github.com/rust-lang/cargo
//! 
//! Voxell Paladynee 2025

// +- Rust Editor.exe -------------------------------+
// | 38 |                                            |
// | 39 | fn main() {                                |
// | 40 | --------- `main` is a special function!    |
// +----|--------------------------------------------+
//
// The entry point of our program, named `main`. The standard library will automatically call this function
// upon program invocation, so it is a special function. It can return any value that implements the
// `Terminate` trait from the standard library. This function returns nothing, and nothing is one such type.
// More on `impl`ementations, `functions` and the `nothing` type later.
// [main]: https://en.wikipedia.org/wiki/Entry_point
fn main() {
    // We're inside the `main` function, and we encounter our first scope with a non-'static lifetime.
    // Anything declared in this function, like variables, do not "live" after main exits.
    // More on `lifetimes`, `'static` and `scopes` later.

    // The standard library and user-written types that have `Drop` implementations
    // may do some cleanup after the main function exits, after then which the program
    // terminates automatically.
    // More on `Drop` later.

    //      the &str type:
    //     +--------+--------+
    //     | 0x0123 |     13 |
    //     +--------+--------+
    //          |
    //          v
    //-----+--------+--------+--------+--------+--------+--------+----
    // ??? |    'H' |    'e' |    'l' |    'l' |    'o' |    ' ' | ...
    //-----+--------+--------+--------+--------+--------+--------+----
    //      hello.exe contents

    // Here we define a `string literal` with an inferred size, and bind it to the variable
    // named `string`. The string literal actually has 'static lifetime, because it is actually
    // a pointer to the to-be-compiled binary program, which exists regardless of whether we're
    // inside `main` or not. Therefore, Rust allows us to make values with 'static lifetimes.

    // The `&str` type (called a "string slice", read as "ref str") refers to a compile-time
    // guaranteed `UTF-8` valid sequence of bytes, therefore it is always safe for us to write
    // this value to a terminal.
    // More on `referring to` later.
    // [UTF-8]: https://en.wikipedia.org/wiki/UTF-8
    // [string literal]: https://en.wikipedia.org/wiki/String_literal
    let string: &'static str = "Hello, world!";

    // Here we invoke the `println!` macro, which is included with the Rust standard library
    // (std) implicit prelude. It takes a format string, and a variable amount of arguments
    // to print some values to the standard output, followed by a new line (`\n`) byte.
    // The println! function may panic if writing or flushing standard output fails.
    // More on the `prelude` and macros later.

    // The first argument to the `println!` macro, the format string, has its own rules
    // and syntax for interpolating strings. Internally, it calls to the `format_args!` macro,
    // which is the macro that actually handles the formatting itself. Other than that, `println!`
    // is only concerned with printing the result of `format_args!` to the standard output.
    // +-------------------------------------------------+
    // | The format string                         _ o x |
    // +----|--------------------------------------------+
    // | 95 | "{}"                                       |
    // |    | ^^^^ the format string                     |
    // +----|--------------------------------------------+

    // The second argument, the binding `string` which refers to our variable with type &str,
    // implements the `Display` trait from the `std::fmt` module, which, when implemented on any
    // value denotes how the value should be formatted to a target `String` for end-user viewing
    // purposes. This trait is what `println!` actually looks for when it's given a value as an
    // argument. For a simple `&str` type, this implementation is provided by `std` by default and
    // it directly writes the bytes of the underlying string to the target buffer. Writing these
    // raw bytes is safe because the compiler guarantees that any value with type &str has this property.
    println!("{}", string);

    // Normally, `string` along with any other value available in the scope would be `Drop`ped after
    // this scope ends at the next close bracket (}), but because string slices don't implement `Drop`
    // (they don't have a "drop glue"), no additional action will be taken by the compiler to get rid of
    // that value. after that the standard library will do some internal cleanup as an implementation
    // detail and terminate the program.
}

