# error_handling
https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html

Rust has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters 
an unrecoverable error.

If the Result value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the Result is the `Err` variant, `unwrap` will call the `panic!` macro.

The `expect` method is similar to `unwrap` but also lets us choose the `panic!` error message. 

It's best to choose an error return type that corresponds to the error value(s) of any operation(s) that are called in the function. The `?` operator automatically converts error types into the type returned in the current function (as long as a `From` trait is defined for that error type). `?` can only be used in functions that return a `Result<T, E> type`.
