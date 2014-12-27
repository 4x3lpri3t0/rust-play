fn main() {
    print_number(5);

    print_sum(5, 6);

    add_one(100);
}

fn print_number(x: int) {
    println!("x is: {}", x);
}

fn print_sum(x: int, y: int) {
    println!("sum is: {}", x + y);
}

// Unlike let, you must declare the types of function arguments.

// What about returning a value?
fn add_one(x: int) -> int {
    x + 1
}
// Our function claims to return an int,
// but with a semicolon, it would return () instead.

// But what about early returns? Rust does have a keyword for that, return:
fn foo(x: int) -> int {
    if x < 5 { return x; }

    x + 1
}

// Using a return as the last line of a function works, but is considered poor style:
fn ugly(x: int) -> int {
    if x < 5 { return x; }

    return x + 1; // :S
}