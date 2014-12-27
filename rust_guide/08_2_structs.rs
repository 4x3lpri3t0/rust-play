fn main() {
    // structs give each element a name, called 'field'
    let origin = Point { x: 0i, y: 0i }; // origin: Point

    println!("The origin is at ({}, {})", origin.x, origin.y);

    let mut mutable_point = Point { x: 0i, y: 0i };

    mutable_point.x = 5;

    println!("The point is at ({}, {})", mutable_point.x, mutable_point.y);
}

// structs begin with a capital letter and are also camel cased: PointInSpace
struct Point {
    x: int,
    y: int,
}