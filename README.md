# Multiple Mutable Borrows in Rust
This example demonstrates a common error in Rust: creating multiple mutable borrows of the same variable.  Rust's borrow checker prevents this to ensure memory safety and avoid data races.

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file provides a corrected version.

This is a great example of how Rust's borrow checker helps prevent common concurrency bugs early in development.