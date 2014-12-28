fn main() {
    
    let x = 5i;
    let y = 8i;

    // location | value
    // 0xd3e030 | 5
    // 0xd3e028 | 8

    // When we refer to x, we get the corresponding value.
    // Hence, x is 5.

    // In some languages, there is just one type of 'pointer,'
    // but in Rust, we have many types.

    // In this case, we'll use a Rust reference, which is the simplest kind of pointer:

    let x = 5i;
    let y = 8i;
    let z = &y;

    // location | value
    // 0xd3e030 | 5
    // 0xd3e028 | 8
    // 0xd3e020 | 0xd3e028

    // z has the type &int

    // We can print this location using the {:p} format string:
    println!("{:p}", z);
    // This would print 0xd3e028, with our fictional memory addresses.

    // Because int and &int are different types, we can't, for example, add them together:
    // println!("{}", x + z); => ERROR

    // We can dereference the pointer by using the * operator.
    // Dereferencing a pointer means accessing the value at the location stored in the pointer:
    println!("{}", x + *z);

    // ----

    // You can't tell at compile time how much memory to allocate,
    // so you've gotta use a pointer to point at the memory where it will be allocated,
    // and deal with it at run time.

    // ----

    // Aliasing can be an issue.
    // Two pointers are said to alias when they point at the same location in memory.
}