// This enum has two variants, one of which has a value:
enum OptionalInt {
    Value(int),
    Missing,
}

// You can also have any number of values in an enum:
enum OptionalColor {
    Color(int, int, int),
    Missing,
}

// And you can also have something like this:
enum StringResult {
    StringOK(String),
    ErrorReason(String),
}

fn main() {
    use StringResult::StringOK;
    use StringResult::ErrorReason;

    enum Ordering {
        Less,
        Equal,
        Greater,
    }

    let x = 5i;
    let y = 10i;

    let ordering = cmp(x, y); // ordering: Ordering

    if ordering == Less {
        println!("less");
    } else if ordering == Greater {
        println!("greater");
    } else if ordering == Equal {
        println!("equal");
    }
}

fn cmp(a: int, b: int) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn respond(greeting: &str) -> StringResult {
    if greeting == "Hello" {
        StringResult::StringOK("Good morning!".to_string())
    } else {
        StringResult::ErrorReason("I didn't understand you!".to_string())
    }
}