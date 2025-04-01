//! Rust program to print hello world.
//!
//! This program, on invocation, prints the bytes "Hello, world!" to the standard output
//! buffer and flushes it, and then immediately terminates. The program may panic if
//! writing to the standard output fails
//!
//! Usage: hello_world.exe
//!
//! Voxell Paladynee 2025

// the entry point of our program. the standard library will call this function upon program
// invocation. it can return any value that implements `Terminate`. `()` is one of them,
// so we don't return anything.
fn main() {
    // we're inside the main function, and we encounter our first scope with a non-'static lifetime.
    // anything declared in this function does not live after main exits.

    // the standard library and the `Drop` implementations of some user-written structs
    // may do some cleanup after the main function ends, after then which the program
    // program terminates.

    // here we define a string literal with an inferred size. the string literal actually
    // has 'static lifetime, because it is actually a pointer to the to-be-compiled binary,
    // which exists regardless of whether the program is running or not. therefore,
    // Rust allows us to make values with 'static lifetimes.

    // the `&str` type holds a compile-time guaranteed UTF-8 valid sequence of bytes,
    // therefore it is always safe for us to push this value to the standard output.
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
