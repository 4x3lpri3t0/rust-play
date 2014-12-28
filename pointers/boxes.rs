// Most of the types we've seen so far have a fixed size or number of components.

// The compiler needs this fact to lay out values in memory.

// However, some data structures, such as a linked list, do not have a fixed size.

// You might think to implement a linked list with an enum that's either a Node
// or the end of the list (Nil), like this:

// enum List {
//    Node(u32, List),
//    Nil
// }

// But the compiler complains that the type is recursive, that is, it could be arbitrarily large.

// To remedy this, Rust provides a fixed-size container called a box that can hold any type.

// You can box up any value with the box keyword.

// Our boxed List gets the type Box<List>:

enum List {
    Node(u32, Box<List>),
    Nil
}

fn main() {
    let list = List::Node(0, box List::Node(1, box List::Nil));


    // A box dynamically allocates memory to hold its contents.

    // The great thing about Rust is that that memory is automatically, efficiently,
    // and predictably deallocated when you're done with the box.

    // A box is a pointer type, and you access what's inside using the * operator,
    // just like regular references.

    {
        let x = box 5i;
        println!("{}", *x); // Prints 5
    }
}

// The great thing about boxes is that we don't have to manually free this allocation!
// Instead, when x reaches the end of its lifetime – in this case, when it goes out of scope
// at the end of the block – Rust frees x.

// This isn't because Rust has a garbage collector (it doesn't).
// Instead, by tracking the ownership and lifetime of a variable (with a little help from you,
// the programmer), the compiler knows precisely when it is no longer used.

// The Rust code above will do the same thing as the following C code:

// {
//     int *x = (int *)malloc(sizeof(int));
//     if (!x) abort();
//     *x = 5;
//     printf("%d\n", *x);
//     free(x);
// }