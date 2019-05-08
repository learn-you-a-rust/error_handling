# error_handling
https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html

Rust has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters 
an unrecoverable error.

If the Result value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the Result is the `Err` variant, `unwrap` will call the `panic!` macro.
