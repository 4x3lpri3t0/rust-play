// Closure:

fn main() {
    let add_one = |x| { 1i + x };

    println!("The sum of 5 plus 1 is {}.", add_one(5i));

    // We create a closure using the |...| { ... } syntax,
    // and then we create a binding so we can use it later.

    // Note that we call the function using the binding name
    // and two parentheses, just like we would for a named function.

    // Let's compare syntax. The two are pretty close:
    let add_one = |x: int| -> int { 1i + x };
    fn  add_one   (x: int) -> int { 1i + x }

    // Closures infer their argument and return types.

    // This is different form named functions, which default to returning unit -> ()

    // There's one big difference between a closure and named functions,
    // and it's in the name: a closure "closes over its environment" :
    let x = 5i;

    let printer = || { println!("x is: {}", x); };

    printer(); // prints "x is: 5"

    // The || syntax means this is an anonymous closure that takes no arguments.

    // A closure has access to variables in the scope where it's defined.
    // The closure borrows any variables it uses, so this will error:
    let mut x = 5i;

    let printer = || { println!("x is: {}", x); };

    x = 6i; // error: cannot assign to 'x' because it is borrowed
}