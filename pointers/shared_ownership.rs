struct Car {
    name: String,
}

struct Wheel {
    size: int,
    owner: Car,
}

fn main() {
    let car = Car { name: "DeLorean".to_string() };

    for _ in range(0u, 4) {
        Wheel { size: 360, owner: car };
    }
}

// We try to make four Wheels, each with a Car that it's attached to.
// But the compiler knows that on the second iteration of the loop, there's a problem:

// We need our Car to be pointed to by multiple Wheels.
// We can't do that with Box<T>, because it has a single owner.
// We can do it with Rc<T> instead:

// See shared_ownership_rc.rs