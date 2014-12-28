fn main() {
    hello::print_hello();
}

mod hello {
    pub fn print_hello() {
        println!("Hello, world!");
    }
}

// Modules allow you to split up your program into nice neat boxes of functionality,
// grouping common things together, and keeping different things apart.

// Usage of the pub keyword is sometimes called 'exporting',
// because we're making the function available for other modules.