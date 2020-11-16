use ::std::*;
use colorful::Color;
use colorful::Colorful;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;

#[macro_use]
extern crate colorful;

fn select_menu() -> std::io::Result<()> {
    let items = vec!["Today's goal", "Write"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything")
    }

    Ok(())
}

fn main() {
    let mut name = String::new();
    println!("Hello there");
    println!("Please enter your name: ");
    let read_name = std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}", name.gradient(Color::Red));
    println!("{}", "Welcome to mydia!".gradient(Color::Green)); 

    select_menu();
}
