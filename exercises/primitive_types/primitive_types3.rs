// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

use std::convert::TryInto;

fn main() {
    let a: [&str; 100] = vec!["wow"; 100].try_into().expect("uh oh");

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
