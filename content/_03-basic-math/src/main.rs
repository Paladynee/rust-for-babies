// what you see below is a function signature. it declares a function that denotes a
// set of behaviors in its body that will be executed with the given arguments when it
// is invoked. a function invocation is often called a "function call".
//
// a function signature can tell us a lot of things about the function itself. the most
// prominent of these features is the name of the function, its arguments, and its return type.
// here, our function signature denotes we take an argument `a` of type `i32`, `b` of type `i32`
// and return another `i32`.
//
// the type `i32` denotes that a variable with this type is a signed 32-bit integer. the
// integer types in Rust use 2's complement, so the highest bit of the signed integer is
// the sign bit:
//
//           vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv data bits
//  17 = 0b 00000000000000000000000000100001
//          ^ sign bit
//
// there are many other aspects to function signatures and types, but we will not touch on them
// on this chapter.
fn add_two_numbers(a: i32, b: i32) -> i32 {
    // we're now in the body of the function `add_two_numbers`, which is also a new
    // "scope" with a different lifetime than of function `main`. the variables
    // declared in the body of this function are cleared upon the exit of this
    // function, so when this function `return`s they are no longer accessible,
    // because they will be freed.

    // here is a `let` statement that binds a value to an identifier. `let` statements
    // allow us to achieve the simplest aspect of programming, saving the result of some
    // operation for later use. in this example, our identifier is `result`, and after
    // `result` is bound to a value, it is allowed to be used later in the code.
    //
    // our value here is whatever `a + b` evaluates to. but we first need to know what
    // exactly does it mean to add things. the `Add` trait from the standard library
    // describes how the addition of two things will behave, and the implementation
    // of the `Add` trait will "overload" the `+` operator, as seen below. for
    // integer types in Rust, this trait is implemented by default, so we can
    // perform addition and other similar mathematical operations on numeric values
    // of the same type.
    let result = a + b;

    // the `return` statement allows us to "return" values from the current
    // function body to the "caller". caller in this context means wherever
    // you invoked (or called) the function at. for example, this `return`
    // statement will return the value bound to the identifier `result`
    // to the caller, which happens to be the result of our addition.
    return result;
}

// here's another function with a lot of arguments, but no return value. in Rust,
// when function signatures omit their return type, it is automatically inferred
// that the function returns the `()` type, which is an empty tuple of nothing.
// it is nothingness itself, it carries no data, and takes exactly 0 bytes in
// memory to represent it. functions returning this type don't evaluate
// to a meaningful value.
//
// the `isize` type may look weird at first, but you just need to know that it
// is a signed integer for now. (isize for signed, usize for unsigned.)
fn many_operations(a: u32, b: u8, _c: i16, _d: isize) {
    // as you can see, we used a new keyword `as` to "cast" the variable `b` of
    // type `u8` into the type `u32`. this is referred to as "as casting", and
    // as casting will retain the numerical value of the operand, so it is not
    // so much slapping a "this is a u32" sign on a value and calling it a day.
    // as casting is only implemented for primitives in Rust, which numbers are
    // a part of.
    //
    // if we had tried to add 2 numbers of different types, we would get a
    // compilation error, because Rust doesn't know how to add 2 numbers of
    // different types, so we reached out to "as casting" for help.
    //
    // this following operation isn't bound to any variable, so it is discarded.
    a + b as u32;

    // here is a similar operation, but as casting `a` into the type `u8` instead.
    // as casting bigger types into smaller types will "truncate" the value,
    // discarding the higher bits.
    b + a as u8;

    // as you can see, we're about to exit the function body and yet there is
    // no sign of any `return` or the use of the variables `_c` and `_d`.
    // in Rust, whenever any function returns a value of type `()`, the function
    // doesn't have to explicitly return anything, but is allowed to do so.
    // also in Rust, whenever variables are not planned to be used (for whatever
    // reason), they are bound to identifiers starting with the underscore
    // character ("_").
}

// once again, our program entrance point, which the standard library calls automatically for us.
fn main() {
    // what you see here is yet another `let` statement but for the value, it is
    // something different this time: a function call. in here, you as the caller
    // invoke the function named `add_two_numbers`, pass it the values `69` for `a`
    // and `420` for `b` and wait for it to return back to you. the function does
    // arbitrary things, and finally returns a value back to you, which in this
    // case is the addition of `69` and `420`. this returned value is then
    // bound to the identifier with the name `my_value`.
    let my_value = add_two_numbers(69, 420);

    // what you see here is a macro call as can bee seen from the bang after the macro name
    // `assert`. macro calls are very much like function calls, but they are a little bit
    // more special. we will touch on these little guys later.
    //
    // the `assert!` macro included in the implicit standard library prelude, when invoked,
    // will check whether the given condition is upheld, and if not, will panic the program.
    // in this case we're asserting that the value bound to the identifier `my_value` is exactly
    // `489` or we abort the program.
    assert!(my_value == 489);

    // here is another function call, but this time for the function `many_operations`.
    // the `many_operations` function does not return any meaningful value, so it is
    // pointless to bind the result of it to an identifier. so this function call does
    // not evaluate to anything, and is not bound to anything.
    many_operations(0, 1, 2, 3);

    // none of the integer types in Rust implement `Drop`, so we have no drop glue, and our `main`
    // function ends here, and our program terminates.
}
