fn main() {
    let mut x = 5u;
    let mut done = false;

    // Rust has two keywords to help us with modifying iteration: break and continue.
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; }
    }

    // continue goes to the next iteration:
    for x in range(0i, 10i) {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }
}