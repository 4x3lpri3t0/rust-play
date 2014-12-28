fn main() {
    let y = &5i;    // -+ y goes into scope
                    //  |
    // stuff        //  |
                    //  |
}                   // -+ y goes out of scope

// ------------------------------------------------------

// Adding in our Foo:

struct Foo<'a> {
    x: &'a int,
}

fn main() {
    let y = &5i;          // -+ y goes into scope
    let f = Foo { x: y }; // -+ f goes into scope
    // stuff              //  |
                          //  |
}                         // -+ f and y go out of scope

// ------------------------------------------------------

// Our f lives within the scope of y, so everything works.
// What if it didn't? This code won't work:

struct Foo<'a> {
    x: &'a int,
}

fn main() {
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5i;          // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}                             // -+ x goes out of scope

// As you can see here, the scopes of f and y are smaller than the scope of x.
// But when we do x = &f.x, we make x a reference to something that's about to go out of scope.

// Named lifetimes are a way of giving these scopes a name.
// Giving something a name is the first step towards being able to talk about it.