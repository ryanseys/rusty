use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");

    io::stdout().flush().expect("flush failed.");

    // read from stdin
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    println!("Hello, {}!", input);
}
