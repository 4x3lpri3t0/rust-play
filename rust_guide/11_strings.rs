fn main() {
    // Rust has two main types of strings: &str and String.
    // &str => 'string slice'
    // String literals are of the type &str:
    let string = "Hello there."; // string: &str

    // This string is statically allocated, meaning that it's saved
    // inside our compiled program, and exists for the entire duration it runs.

    // The string binding is a reference to this statically allocated string.
    // String slices have a fixed size, and cannot be mutated.

    // ----

    // A String, on the other hand, is an in-memory string.
    // This string is growable, and is also guaranteed to be UTF-8.
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    // You can get a &str view into a String with the as_slice() method:
    fn takes_slice(slice: &str) {
        println!("Got: {}", slice);
    }

    let s = "Hello".to_string();
    takes_slice(s.as_slice());

    // To compare a String to a constant string, prefer as_slice()...
    fn compare_using_as_slice(string: String) {
        if string.as_slice() == "Hello" {
            println!("yes");
        }
    }

    // ... over to_string():
    fn compare_using_to_string(string: String) {
        if string == "Hello".to_string() {
            println!("yes");
        }
    }

    // Viewing a String as a &str is cheap, but converting the &str to a String
    // involves allocating memory.
    // No reason to do that unless you have to!
}

// Just remember that Strings allocate memory and control their data,
// while &strs are a reference to another string