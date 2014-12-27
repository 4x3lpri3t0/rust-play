fn cmp(a: int, b: int) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

enum OptionalInt {
    Value(int),
    Missing,
}

fn main() {
    let x = 5i;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let x = 5i;
    let y = 10i;

    match cmp(x, y) {
        Less => println!("less"),
        Greater => println!("greater"),
        Equal => println!("equal"),
    }

    // match expressions also allow us to get the values contained
    // in an enum (also known as destructuring) as follows:
    let x = OptionalInt::Value(5);
    let y = OptionalInt::Missing;

    match x {
        OptionalInt::Value(n) => println!("x is {}", n),
        OptionalInt::Missing => println!("x is missing!"),
    }

    match y {
        OptionalInt::Value(n) => println!("y is {}", n),
        OptionalInt::Missing => println!("y is missing!"),
    }

    let x = 5i;
    let y = 10i;

    // Also:
    println!("{}", match cmp(x, y) {
        Less    => "less",
        Greater => "greater",
        Equal   => "equal",
    });
}
