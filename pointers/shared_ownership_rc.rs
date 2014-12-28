use std::rc::Rc;

struct Car {
    name: String,
}

struct Wheel {
    size: int,
    owner: Rc<Car>,
}

fn main() {
    let car = Car { name: "DeLorean".to_string() };

    let car_owner = Rc::new(car);

    for _ in range(0u, 4) {
        Wheel { size: 360, owner: car_owner.clone() };
    }
}

// We wrap our Car in an Rc<T>, getting an Rc<Car>, and then use the clone() method
// to make new references.

// We've also changed our Wheel to have an Rc<Car> rather than just a Car.

// This is the simplest kind of multiple ownership possible.