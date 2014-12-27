// Here's a simple program that reads some input, and then prints it back out:

// It's considered better practice to not import individual functions,
// but to import the module, and only use one level of qualification:
use std::io;

fn main() {
    println!("Type something!");

    let input = io::stdin()
                  .read_line()
                  .ok()
                  .expect("Failed to read line");

    println!("{}", input);

    // std::io::stdin();
    // Calls a function, stdin(), that lives inside the std::io module

    // .read_line()
    // The read_line() method can be called on the result of stdin()
    // to return a full line of input.

    // .ok().expect("Failed to read line");
    // read_line returns a type very similar to our OptionalInt: an IoResult<T>
    // Rust provides a method on these IoResult<T>s called ok(),
    // which does the same thing as our match statement but assumes that we have a valid value

    // We then call expect() on the result,
    // which will terminate our program if we don't have a valid value.


}