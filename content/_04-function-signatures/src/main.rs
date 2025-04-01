// ignore this line, we will touch upon these later when exploring "modules".
use std::fmt::Display;

// here, we again have our main function, with the signature `fn main()`. it doesn't return anything.
fn main() {
    println!("Hello, world!");
}

// the first new thing we will learn about function signatures is their visibility.
// functions (and many other items in Rust) can have a visibility specifier, which
// denotes where you can access those items from. we will touch upon visibility
// and modules later.
//
// this is the `pub` qualifier, which is the most public, everyone (including
// other people who can see this module) can call this function.
pub fn call_me() {}
// this is the `pub(crate)` qualifier. you can call this function anywhere you
// want, but not others.
pub(crate) fn call_me_myself() {}
mod ignore {
    // this is the `pub(super)` qualifier. you can call this function only in the current,
    // and the parent module
    pub(super) fn call_me_daddy() {}
}

// the second thing we'll learn about a function's signature is its constness.
// functions denoted with `const` can be called at compile time, their values
// evaluated at compile time, their results able to be bound to `const` or
// `static` variables, and much more.
//
// but with great power, comes great responsibility. you can't call a non-const
// function inside a `const` function. but luckily, the standard library
// and the Rust language provides a lot of tools for us to work in a
// const function.
//
// a function denoted with `const` does not mean it will always be called
// at compile time. const functions can be called at runtime too, for example
// when their arguments are only known in runtime.
const fn calculate() -> i32 {
    return 37;
}

// the third thing we'll learn about a function's signature is its unsafety.
// functions marked with `unsafe` can do arbitrarily bad things if you don't
// pay attention to its safety contracts. in other languages such as C,
// every function is unsafe, and you can always shoot yourself in the foot.
// in Rust, if you never use unsafe, it is impossible for you to shoot yourself
// in the foot, because the compiler will stop you before you do it.
//
// we'll touch on unsafety later.
//
// the next lines starting with triple forward slashes ("///") denote a documentation
// comment, shortened as doc comment, and they carry information regarding the function
// itself for humans to read. it can include information about the function such as
// explanation for the return types, whether it can error, or panic, or safety
// contracts if it is unsafe. in your favourite editor, if you have rust-analyzer
// or similar installed, you can hover over the function's name to see the
// function signature and its doc comments in a prettified manner. pretty cool!

/// Reads a `u8` value from a given pointer.
///
/// # Safety
///
/// `ptr` must be a non-null, well-aligned pointer valid for reads
/// of type `u8`. Behavior is otherwise undefined.
unsafe fn deref_pointer(ptr: *const u8) -> u8 {
    // Safety: caller upholds the contract.
    unsafe { *ptr }
}

// one final thing we will learn about function signatures is generic arguments.
// generics are a bit of a complex topic, so we'll refrain from explaining them
// here. for now, you just need to know that they act as fancy compile-time
// types that satisfy some bounds that are written explicitly, and can be used
// in the function signature itself, or the body. there can be any amount of
// generic arguments of a function. generic arguments can often be inferred
// by the compiler based on the concrete types you give it at callsite.
//
// this function takes a generic type `T`, and declares the function such that
// it takes in an argument of type `T` named `value`, and returns a value
// of type `T` back. the type has a bound, which states that it has to implement
// `Display`.
fn print_this<T: Display>(value: T) -> T {
    println!("{}", value);
    return value;
}
