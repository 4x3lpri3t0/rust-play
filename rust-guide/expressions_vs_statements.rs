// Expressions return a value, statements do not.

// In Rust, if is an expression, which means that it returns a value.

fn main() {
    let x = 5i;

    let y: int = if x == 5i { 10i; } else { 15i; };
    // Note the semicolons after the 10 and 15.
    // Rust will give us the following error:

    // error: mismatched types: expected `int` but found `()` (expected int but found ())

    // We expected an integer, but we got ().
    // () is pronounced 'unit', and is a special type in Rust's type system.
    // In Rust, () is not a valid value for a variable of type int.
    // It's only a valid value for variables of the type (), which aren't very useful.
    
    // Remember how we said statements don't return a value?
    // Well, that's the purpose of unit in this case.
    // The semicolon turns any expression into a statement by throwing away its value
    // and returning unit instead.
}