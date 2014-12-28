extern crate testing;
use testing::add_three_times_four;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
