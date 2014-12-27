fn main() {
    // Two-length tuple
    let x = (1i, "hello");

    // Same code, with type annotated:
    let x: (int, &str) = (1, "hello");

    // Destructuring let:
    let(x, y, z) = (1i, 2i, 3i);

    println!("x is {}", x);

    // You can assign one tuple into another, if they have the same arity and contained types:
    let mut x = (1i, 2i);
    let y = (2i, 3i);

    x = y;

    // Check for equality with ==
    let x = (1i, 2i, 3i);
    let y = (2i, 2i, 4i);

    if x == y {
        println!("yes");
    } else {
        println!("no");
    } // This will print no, because some of the values aren't equal.

    let (x, y) = next_two(5i);
    println!("x, y = {}, {}", x, y);
}

// One other use of tuples is to return multiple values from a function:
fn next_two(x: int) -> (int, int) { (x + 1i, x + 2i) }
// Even though Rust functions can only return one value,
// a tuple is one value, that happens to be made up of two.