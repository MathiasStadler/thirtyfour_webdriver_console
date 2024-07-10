// FROM HERE
// https://fitech101.aalto.fi/programming-languages/rust/8-interaction-input-and-os/
use std::io;
fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    println!("Hi, {name}!");
}
