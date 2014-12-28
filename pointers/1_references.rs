fn main() {
    let x = 5i;
    let y = &x; // y is a reference to x

    // To dereference (get the value being referred to rather than the reference itself) y, we use *:
    assert_eq!(5i, *y);

    // You can declare that functions take a reference:
    fn add_one(x: &int) -> int { *x + 1 }

    assert_eq!(6, add_one(&5)); // we can make a reference from a literal by applying & as well

    // Because references are immutable, you can have multiple references that alias:
    let x = 5i;
    let y = &x;
    let z = &x;

    // We can make a mutable reference by using &mut instead of &:
    let mut x = 5i; // x must also be mutable
    let y = &mut x;
}