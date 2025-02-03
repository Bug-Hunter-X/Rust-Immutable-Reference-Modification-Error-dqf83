# Rust Immutable Reference Modification Bug

This repository demonstrates a common error in Rust: attempting to modify a variable through an immutable reference.  The code attempts to modify the value of `x` through an immutable reference `z`. Rust's borrow checker prevents this, ensuring memory safety.