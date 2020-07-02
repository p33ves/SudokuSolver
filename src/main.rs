use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please enter the sudoku puzzle:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");

    println!("Your input was : {}", input.trim().chars().count());

    match 81.cmp(&input.trim().chars().count()) {
        Ordering::Less => println!("This seems too long!"),
        Ordering::Greater => println!("You haven't typed the full puzzle!"),
        Ordering::Equal => println!("Validating input..."),
    }
}
