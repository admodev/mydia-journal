use ::std::*;

fn main() {
    let mut name = String::new();
    println!("Hello there");
    println!("Please enter your name: ");
    let read_name = std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}", name);
    println!("Welcome to mydia!");
}
