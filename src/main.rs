use std::io::{self, Read};

fn main() {
    // Read the event from stdin (Lambda passes the event payload via a temp file)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Process event (here we just echo it)
    let response = format!("Hello from Rust Lambda! Event was: {}", input);

    println!("{}", response);  // stdout will be the response
}
