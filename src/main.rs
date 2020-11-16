use ::std::*;
use colorful::Color;
use colorful::Colorful;

#[macro_use]
extern crate colorful;

fn main() {
    let mut name = String::new();
    println!("Hello there");
    println!("Please enter your name: ");
    let read_name = std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}", name.gradient(Color::Red));
    println!("{}", "Welcome to mydia!".gradient(Color::Green)); 
}
