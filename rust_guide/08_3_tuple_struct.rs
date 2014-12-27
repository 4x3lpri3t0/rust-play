fn main() {
    // Tuple structs do have a name, but their fields don't:
    struct ColorTupleStruct(int, int, int);
    struct PointTupleStruct(int, int, int);

    // These two will not be equal, even if they have the same values:
    let black = ColorTupleStruct(0, 0, 0);
    let origin = PointTupleStruct(0, 0, 0);

    // It is almost always better to use a struct than a tuple struct:
    struct ColorStruct {
        red: int,
        blue: int,
        green: int,
    }

    struct PointStruct {
        x: int,
        y: int,
        z: int,
    }
    // Now, we have actual names, rather than positions.

    // There is one case when a tuple struct is very useful, though,
    // and that's a tuple struct with only one element.
    
    // We call this a 'newtype,' because it lets you create a new type
    // that's a synonym for another one:

    struct Inches(int);
    
    let length = Inches(10);

    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}