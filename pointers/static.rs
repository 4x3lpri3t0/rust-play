// The lifetime named 'static' is a special lifetime.
// It signals that something has the lifetime of the entire program.

let x: &'static str = "Hello, world.";

// Another example are globals:

static FOO: int = 5i;
let x: &'static int = &FOO;