fn main() {
    // ARRAY

    // A fixed-size list of elements of the same type.
    // By default, an array is inmutable.

    let a = [1i, 2i, 3i];
    let mut m = [1i, 2i, 3i];

    // You can create an array with a given number of elements,
    // all initialized to the same value, with [val, ..N] syntax:

    let a = [0i, ..20];

    // To get the number of elements in an array use a.len()
    // Use a.iter() to iterate over them with a for loop

    let a = [1i, 2, 3]; // Only the first element needs a type suffix

    println!("a has {} elements", a.len());
    for e in a.iter() {
        println!("{}", e);
    }

    // You can access a particular element of an array with subscript notation:

    let names = ["Graydon", "Axel", "Nick"]; // names: [&str, 3]

    println!("The second name is: {}", names[1]);

    // If you try to use a subscript that is not in the array,
    // you will get an error: array access is bounds-checked at run-time.

    // ----

    // VECTOR

    // A vector is a dynamic or "growable" array, implemented as the standard library type Vec<T>
    // Vectors are to arrays what String is to &str.

    // You can create them with the vec! macro:

    let v = vec![1i, 2, 3]; // v: Vec<int>

    let mut nums = vec![1i, 2, 3]; // mut nums: Vec<int>

    nums.push(4);

    println!("The length of nums is now {}", nums.len()); // Prints 4

    // ----

    // SLICE

    // A slice is a reference to (or "view" into) an array.

    // They are useful for allowing safe, efficient access to a portion of an array without copying.
    // For example, you might want to reference just one line of a file read into memory.

    // By nature, a slice is not created directly, but from an existing variable.
    // Slices have a length, can be mutable or not, and in many ways behave like arrays:

    let a = [0i, 1, 2, 3, 4];
    let middle = a.slice(1, 4); // A slice of a: just the elements [1, 2, 3]

    for e in middle.iter() {
        println!("{}", e);
    }

    // You can also take a slice of a vector, String, or &str, because they are backed by arrays.
    // Slices have type &[T]
}