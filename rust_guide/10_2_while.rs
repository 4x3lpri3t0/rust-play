fn main() {
    let mut x = 5u;       // mut x: uint
    let mut done = false; // mut donde: bool

    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { done = true; }
    }

    // for looping infinitely use 'loop'
}