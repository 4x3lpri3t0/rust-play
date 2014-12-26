fn main() {
    let x = 5i;
    // In many languages, this is called a 'variable.'
    // But Rust's bindings have a few tricks up their sleeves.
    // Rust has a very powerful feature called 'pattern matching'...

    // Left hand side of a let expression is a full pattern, not just a variable name:
    let (x, y) = (1i, 2i);

    // Bindings are immutable.

    // If you want a binding to be mutable, you can use mut:
    let mut x = 5i;
    x = 10i;

    // Bindings are required to be initialized with a value
    // before you're allowed to use them:
    // let x; // => ERROR

    // Giving it a type will compile, though:
    let x: int;
}