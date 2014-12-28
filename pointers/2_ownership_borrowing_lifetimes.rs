fn main() {
    // Whenever a resource of some kind is created,
    // something must be responsible for destroying that resource as well.

    // When you allocate heap memory, you need a mechanism to free that memory.
    // Many languages use a garbage collector to handle deallocation.

    // Rust chooses a different path, and that path is called ownership.
    // Any binding that creates a resource is the owner of that resource.

    // Being an owner affords you some privileges:

    // 1. You control when that resource is deallocated.
    // 2. You may lend that resource, immutably, to as many borrowers as you'd like.
    // 3. You may lend that resource, mutably, to a single borrower.
    
    // But it also comes with some restrictions:

    // 1. If someone is borrowing your resource (either mutably or immutably),
    // you may not mutate the resource or mutably lend it to someone.
    
    // 2. If someone is mutably borrowing your resource, you may not lend it out at all
    // (mutably or immutably) or access it in any way.

    // ----

    // The length of time that the borrower is borrowing the pointer from you is called a lifetime.

    // If two distinct bindings share a pointer, and the memory that pointer points to is immutable,
    // then there are no problems.
    
    // But if it's mutable, the result of changing it can vary unpredictably depending on who happens
    // to access it first, which is called a race condition.
    
    // To avoid this, if someone wants to mutate something that they've borrowed from you,
    // you must not have lent out that pointer to anyone else.

    // Rust has a sophisticated system called the borrow checker to make sure that everyone plays
    // by these rules.
    // At compile time, it verifies that none of these rules are broken.
    // If our program compiles successfully, Rust can guarantee it is free of data races and other
    // memory errors, and there is no runtime overhead for any of this.

    // The borrow checker works only at compile time.

    // If the borrow checker did find a problem, it will report an error and your program will
    // refuse to compile.

    // ----

    let x = 5i; // x is the owner of this integer, which is memory on the stack.

    // other code here...

} // privilege 1: when x goes out of scope, this memory is deallocated

/// this function borrows an integer. It's given back automatically when the
/// function returns.
fn foo(x: &int) -> &int { x }

fn name(x: &int) -> &int {
    // x is the owner of the integer, which is memory on the stack.
    let x = 5i;

    // privilege 2: you may lend that resource to as many borrowers as you like (immutably)
    let y = &x;
    let z = &x;

    foo(&x); // functions can borrow too!

    let a = &x; // more borrowing...

    // ----

    let mut m = 5i;

    // privilege 3: you may lend that resource to a single borrower (mutably)
    let o = &mut x;
}